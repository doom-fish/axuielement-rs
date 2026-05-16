import ApplicationServices
import Foundation

@_silgen_name("AXUIElementPostKeyboardEvent")
private func AXUIElementPostKeyboardEventShim(
    _ application: AXUIElement,
    _ keyChar: UInt16,
    _ virtualKey: UInt16,
    _ keyDown: Bool
) -> AXError

@_cdecl("ax_ui_element_get_type_id")
public func ax_ui_element_get_type_id() -> UInt {
    AXUIElementGetTypeID()
}

@_cdecl("ax_ui_element_create_application")
public func ax_ui_element_create_application(_ pid: Int32) -> UnsafeMutableRawPointer {
    retainObject(AXUIElementCreateApplication(pid))
}

@_cdecl("ax_ui_element_retain")
public func ax_ui_element_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let element: AXUIElement = unretainedObject(handle)
    return retainObject(element)
}

@_cdecl("ax_ui_element_release")
public func ax_ui_element_release(_ handle: UnsafeMutableRawPointer?) {
    releaseObject(handle)
}

@_cdecl("ax_ui_element_get_pid")
public func ax_ui_element_get_pid(
    _ handle: UnsafeMutableRawPointer?,
    _ outPid: UnsafeMutablePointer<Int32>?
) -> Int32 {
    guard let handle, let outPid else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    return AXUIElementGetPid(element, outPid).rawValue
}

@_cdecl("ax_ui_element_set_messaging_timeout")
public func ax_ui_element_set_messaging_timeout(
    _ handle: UnsafeMutableRawPointer?,
    _ timeout: Float
) -> Int32 {
    guard let handle else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    return AXUIElementSetMessagingTimeout(element, timeout).rawValue
}

@_cdecl("ax_ui_element_copy_element_at_position")
public func ax_ui_element_copy_element_at_position(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Float,
    _ y: Float,
    _ outElement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let handle, let outElement else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    var found: AXUIElement?
    let error = AXUIElementCopyElementAtPosition(element, x, y, &found)
    if error == .success, let found {
        outElement.pointee = retainObject(found)
    } else {
        outElement.pointee = nil
    }
    return error.rawValue
}

@_cdecl("ax_ui_element_post_keyboard_event")
public func ax_ui_element_post_keyboard_event(
    _ handle: UnsafeMutableRawPointer?,
    _ keyChar: UInt16,
    _ virtualKey: UInt16,
    _ keyDown: Bool
) -> Int32 {
    guard let handle else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    return AXUIElementPostKeyboardEventShim(element, keyChar, virtualKey, keyDown).rawValue
}
