import ApplicationServices
import Foundation

@_cdecl("ax_system_wide_create")
public func ax_system_wide_create() -> UnsafeMutableRawPointer {
    retainObject(AXUIElementCreateSystemWide())
}

private func copySystemWideElementAttribute(
    _ handle: UnsafeMutableRawPointer?,
    _ attribute: CFString,
    _ outElement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let handle, let outElement else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    var value: CFTypeRef?
    let error = AXUIElementCopyAttributeValue(element, attribute, &value)
    guard error == .success, let value else {
        outElement.pointee = nil
        return error.rawValue
    }
    let object = value as AnyObject
    guard cfTypeID(object) == AXUIElementGetTypeID() else {
        outElement.pointee = nil
        return error.rawValue
    }
    let child = unsafeBitCast(object, to: AXUIElement.self)
    outElement.pointee = retainObject(child)
    return error.rawValue
}

@_cdecl("ax_system_wide_copy_focused_application")
public func ax_system_wide_copy_focused_application(
    _ handle: UnsafeMutableRawPointer?,
    _ outElement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    copySystemWideElementAttribute(handle, "AXFocusedApplication" as CFString, outElement)
}

@_cdecl("ax_system_wide_copy_focused_ui_element")
public func ax_system_wide_copy_focused_ui_element(
    _ handle: UnsafeMutableRawPointer?,
    _ outElement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    copySystemWideElementAttribute(handle, "AXFocusedUIElement" as CFString, outElement)
}

@_cdecl("ax_system_wide_copy_focused_window")
public func ax_system_wide_copy_focused_window(
    _ handle: UnsafeMutableRawPointer?,
    _ outElement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    copySystemWideElementAttribute(handle, "AXFocusedWindow" as CFString, outElement)
}
