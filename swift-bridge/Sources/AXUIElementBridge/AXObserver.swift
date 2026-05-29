import ApplicationServices
import Foundation

public typealias RustObserverCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void

public typealias RustObserverInfoCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void

final class AXObserverBox {
    let observer: AXObserver
    let source: CFRunLoopSource
    let callback: RustObserverCallback?
    let infoCallback: RustObserverInfoCallback?
    var scheduledRunLoop: CFRunLoop?

    init(observer: AXObserver, callback: RustObserverCallback?) {
        self.observer = observer
        self.source = AXObserverGetRunLoopSource(observer)
        self.callback = callback
        self.infoCallback = nil
    }

    init(observer: AXObserver, infoCallback: RustObserverInfoCallback?) {
        self.observer = observer
        self.source = AXObserverGetRunLoopSource(observer)
        self.callback = nil
        self.infoCallback = infoCallback
    }
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

private let observerTrampoline: AXObserverCallback = { observer, element, notification, refcon in
    guard let box = lookupObserver(observer), let callback = box.callback else {
        return
    }
    callback(
        retainObject(box),
        retainObject(element),
        retainObject((notification as String) as NSString),
        refcon)
}

private let observerInfoTrampoline: AXObserverCallbackWithInfo = { observer, element, notification, info, refcon in
    guard let box = lookupObserver(observer), let callback = box.infoCallback else {
        return
    }
    let infoHandle = retainObject(info)
    callback(
        retainObject(box),
        retainObject(element),
        retainObject((notification as String) as NSString),
        infoHandle,
        refcon)
}

@_cdecl("ax_observer_get_type_id")
public func ax_observer_get_type_id() -> UInt {
    AXObserverGetTypeID()
}

@_cdecl("ax_observer_create")
public func ax_observer_create(
    _ pid: Int32,
    _ callback: RustObserverCallback?,
    _ outObserver: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outObserver else {
        return AXError.illegalArgument.rawValue
    }
    var observer: AXObserver?
    let error = AXObserverCreate(pid, observerTrampoline, &observer)
    guard error == .success, let observer else {
        outObserver.pointee = nil
        return error.rawValue
    }
    let box = AXObserverBox(observer: observer, callback: callback)
    registerObserver(box)
    outObserver.pointee = retainObject(box)
    return error.rawValue
}

@_cdecl("ax_observer_create_with_info")
public func ax_observer_create_with_info(
    _ pid: Int32,
    _ callback: RustObserverInfoCallback?,
    _ outObserver: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outObserver else {
        return AXError.illegalArgument.rawValue
    }
    var observer: AXObserver?
    let error = AXObserverCreateWithInfoCallback(pid, observerInfoTrampoline, &observer)
    guard error == .success, let observer else {
        outObserver.pointee = nil
        return error.rawValue
    }
    let box = AXObserverBox(observer: observer, infoCallback: callback)
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
    if let runLoop = box.scheduledRunLoop {
        CFRunLoopRemoveSource(runLoop, box.source, .defaultMode)
        box.scheduledRunLoop = nil
    }
    unregisterObserver(box)
    releaseObject(handle)
}

/// Balances the per-callback `retainObject(box)` performed by the observer
/// trampolines.
///
/// Each delivered notification hands Rust a freshly retained (`+1`) observer
/// handle. The Rust callback must drop that reference once it is done, but it
/// must **not** tear down the run-loop source or unregister the observer the
/// way `ax_observer_release` does — otherwise the observer would stop firing
/// after the first event. This entry point therefore only performs the
/// matching `-1`, leaving the run-loop source scheduled and the registry entry
/// intact. The real teardown still happens exactly once, when the owning Rust
/// `AXObserver` is dropped and calls `ax_observer_release`.
@_cdecl("ax_observer_release_callback")
public func ax_observer_release_callback(_ handle: UnsafeMutableRawPointer?) {
    releaseObject(handle)
}

@_cdecl("ax_observer_schedule_on_current_run_loop")
public func ax_observer_schedule_on_current_run_loop(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    let box: AXObserverBox = unretainedObject(handle)
    guard box.scheduledRunLoop == nil else {
        return
    }
    let runLoop = CFRunLoopGetCurrent()
    CFRunLoopAddSource(runLoop, box.source, .defaultMode)
    box.scheduledRunLoop = runLoop
}

@_cdecl("ax_observer_unschedule_from_run_loop")
public func ax_observer_unschedule_from_run_loop(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    let box: AXObserverBox = unretainedObject(handle)
    guard let runLoop = box.scheduledRunLoop else {
        return
    }
    CFRunLoopRemoveSource(runLoop, box.source, .defaultMode)
    box.scheduledRunLoop = nil
}

@_cdecl("ax_run_current_run_loop")
public func ax_run_current_run_loop() {
    CFRunLoopRun()
}

@_cdecl("ax_stop_current_run_loop")
public func ax_stop_current_run_loop() {
    CFRunLoopStop(CFRunLoopGetCurrent())
}
