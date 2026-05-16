import ApplicationServices
import Foundation

@_cdecl("ax_action_copy_names")
public func ax_action_copy_names(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let handle, let outValue else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    var names: CFArray?
    let error = AXUIElementCopyActionNames(element, &names)
    if error == .success, let names {
        outValue.pointee = retainObject(names)
    } else {
        outValue.pointee = nil
    }
    return error.rawValue
}

@_cdecl("ax_action_copy_description")
public func ax_action_copy_description(
    _ handle: UnsafeMutableRawPointer?,
    _ action: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let handle, let action, let outValue else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    var description: CFString?
    let error = AXUIElementCopyActionDescription(element, String(cString: action) as CFString, &description)
    if error == .success, let description {
        outValue.pointee = retainObject(description)
    } else {
        outValue.pointee = nil
    }
    return error.rawValue
}

@_cdecl("ax_action_perform")
public func ax_action_perform(
    _ handle: UnsafeMutableRawPointer?,
    _ action: UnsafePointer<CChar>?
) -> Int32 {
    guard let handle, let action else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    return AXUIElementPerformAction(element, String(cString: action) as CFString).rawValue
}
