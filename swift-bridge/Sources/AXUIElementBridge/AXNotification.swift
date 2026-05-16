import ApplicationServices
import Foundation

@_cdecl("ax_notification_add")
public func ax_notification_add(
    _ observerHandle: UnsafeMutableRawPointer?,
    _ elementHandle: UnsafeMutableRawPointer?,
    _ notification: UnsafePointer<CChar>?,
    _ refcon: UnsafeMutableRawPointer?
) -> Int32 {
    guard let observerHandle, let elementHandle, let notification else {
        return AXError.illegalArgument.rawValue
    }
    let observerBox: AXObserverBox = unretainedObject(observerHandle)
    let element: AXUIElement = unretainedObject(elementHandle)
    return AXObserverAddNotification(
        observerBox.observer,
        element,
        String(cString: notification) as CFString,
        refcon).rawValue
}

@_cdecl("ax_notification_remove")
public func ax_notification_remove(
    _ observerHandle: UnsafeMutableRawPointer?,
    _ elementHandle: UnsafeMutableRawPointer?,
    _ notification: UnsafePointer<CChar>?
) -> Int32 {
    guard let observerHandle, let elementHandle, let notification else {
        return AXError.illegalArgument.rawValue
    }
    let observerBox: AXObserverBox = unretainedObject(observerHandle)
    let element: AXUIElement = unretainedObject(elementHandle)
    return AXObserverRemoveNotification(
        observerBox.observer,
        element,
        String(cString: notification) as CFString).rawValue
}
