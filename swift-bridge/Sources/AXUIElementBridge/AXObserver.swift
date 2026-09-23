import ApplicationServices
import Foundation

public typealias RustObserverCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void

public typealias RustObserverContextHook = @convention(c) (UnsafeMutableRawPointer?) -> Void

final class AXObserverBox {
    let observer: AXObserver
    let source: CFRunLoopSource
    let callback: RustObserverCallback
    let context: UnsafeMutableRawPointer?
    private let releaseContext: RustObserverContextHook
    private let lock = NSLock()
    private var scheduledRunLoop: CFRunLoop?

    init(
        observer: AXObserver,
        callback: @escaping RustObserverCallback,
        context: UnsafeMutableRawPointer?,
        retainContext: RustObserverContextHook,
        releaseContext: @escaping RustObserverContextHook
    ) {
        self.observer = observer
        self.source = AXObserverGetRunLoopSource(observer)
        self.callback = callback
        self.context = context
        self.releaseContext = releaseContext
        retainContext(context)
    }

    deinit {
        if let scheduledRunLoop {
            CFRunLoopRemoveSource(scheduledRunLoop, source, .commonModes)
        }
        releaseContext(context)
    }

    func schedule(on runLoop: CFRunLoop) {
        lock.lock()
        defer { lock.unlock() }
        guard scheduledRunLoop == nil else {
            return
        }
        CFRunLoopAddSource(runLoop, source, .commonModes)
        scheduledRunLoop = runLoop
    }

    func unschedule() {
        lock.lock()
        let runLoop = scheduledRunLoop
        scheduledRunLoop = nil
        lock.unlock()
        guard let runLoop else {
            return
        }
        CFRunLoopRemoveSource(runLoop, source, .commonModes)
        waitForRunningCallout(on: runLoop)
    }
}

private func waitForRunningCallout(on runLoop: CFRunLoop) {
    guard runLoop !== CFRunLoopGetCurrent(), CFRunLoopCopyCurrentMode(runLoop) != nil else {
        return
    }
    let drained = DispatchSemaphore(value: 0)
    CFRunLoopPerformBlock(runLoop, CFRunLoopMode.commonModes.rawValue) {
        drained.signal()
    }
    CFRunLoopWakeUp(runLoop)
    _ = drained.wait(timeout: .now() + .seconds(2))
}

private let observerRegistryLock = NSLock()
private var observerRegistry: [UInt: AXObserverBox] = [:]

private func observerKey(_ observer: AXObserver) -> UInt {
    UInt(bitPattern: Unmanaged.passUnretained(observer).toOpaque())
}

private func registerObserver(_ box: AXObserverBox) {
    observerRegistryLock.lock()
    observerRegistry[observerKey(box.observer)] = box
    observerRegistryLock.unlock()
}

private func unregisterObserver(_ box: AXObserverBox) {
    observerRegistryLock.lock()
    observerRegistry.removeValue(forKey: observerKey(box.observer))
    observerRegistryLock.unlock()
}

private func lookupObserver(_ observer: AXObserver) -> AXObserverBox? {
    observerRegistryLock.lock()
    let box = observerRegistry[observerKey(observer)]
    observerRegistryLock.unlock()
    return box
}

private func deliverNotification(
    _ observer: AXObserver,
    _ element: AXUIElement,
    _ notification: CFString,
    _ info: CFDictionary?
) {
    guard let box = lookupObserver(observer) else {
        return
    }
    withExtendedLifetime(box) {
        box.callback(
            box.context,
            retainObject(element),
            retainObject((notification as String) as NSString),
            info.map { retainObject($0) })
    }
}

private let observerTrampoline: AXObserverCallback = { observer, element, notification, _ in
    deliverNotification(observer, element, notification, nil)
}

private let observerInfoTrampoline: AXObserverCallbackWithInfo = { observer, element, notification, info, _ in
    deliverNotification(observer, element, notification, info)
}

@_cdecl("ax_observer_get_type_id")
public func ax_observer_get_type_id() -> UInt {
    AXObserverGetTypeID()
}

@_cdecl("ax_observer_create")
public func ax_observer_create(
    _ pid: Int32,
    _ withInfo: Bool,
    _ callback: RustObserverCallback?,
    _ context: UnsafeMutableRawPointer?,
    _ retainContext: RustObserverContextHook?,
    _ releaseContext: RustObserverContextHook?,
    _ outObserver: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outObserver else {
        return AXError.illegalArgument.rawValue
    }
    outObserver.pointee = nil
    guard let callback, let retainContext, let releaseContext else {
        return AXError.illegalArgument.rawValue
    }
    var observer: AXObserver?
    let error = withInfo
        ? AXObserverCreateWithInfoCallback(pid, observerInfoTrampoline, &observer)
        : AXObserverCreate(pid, observerTrampoline, &observer)
    guard error == .success, let observer else {
        return error.rawValue
    }
    let box = AXObserverBox(
        observer: observer,
        callback: callback,
        context: context,
        retainContext: retainContext,
        releaseContext: releaseContext)
    registerObserver(box)
    outObserver.pointee = retainObject(box)
    return error.rawValue
}

@_cdecl("ax_observer_release")
public func ax_observer_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    let box: AXObserverBox = unretainedObject(handle)
    unregisterObserver(box)
    box.unschedule()
    releaseObject(handle)
}

@_cdecl("ax_observer_schedule_on_current_run_loop")
public func ax_observer_schedule_on_current_run_loop(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    let box: AXObserverBox = unretainedObject(handle)
    box.schedule(on: CFRunLoopGetCurrent())
}

@_cdecl("ax_observer_unschedule_from_run_loop")
public func ax_observer_unschedule_from_run_loop(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    let box: AXObserverBox = unretainedObject(handle)
    box.unschedule()
}

@_cdecl("ax_run_current_run_loop")
public func ax_run_current_run_loop() {
    CFRunLoopRun()
}

@_cdecl("ax_stop_current_run_loop")
public func ax_stop_current_run_loop() {
    CFRunLoopStop(CFRunLoopGetCurrent())
}
