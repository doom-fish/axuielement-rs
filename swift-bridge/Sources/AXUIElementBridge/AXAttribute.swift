import ApplicationServices
import Foundation

private func copyNamesArray(
    _ error: AXError,
    _ names: CFArray?,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outValue else {
        return AXError.illegalArgument.rawValue
    }
    if error == .success, let names {
        outValue.pointee = retainObject(names)
    } else {
        outValue.pointee = nil
    }
    return error.rawValue
}

@_cdecl("ax_attribute_copy_names")
public func ax_attribute_copy_names(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let handle else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    var names: CFArray?
    return copyNamesArray(AXUIElementCopyAttributeNames(element, &names), names, outValue)
}

@_cdecl("ax_attribute_is_settable")
public func ax_attribute_is_settable(
    _ handle: UnsafeMutableRawPointer?,
    _ attribute: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<Bool>?
) -> Int32 {
    guard let handle, let attribute, let outValue else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    var settable = DarwinBoolean(false)
    let error = AXUIElementIsAttributeSettable(element, String(cString: attribute) as CFString, &settable)
    outValue.pointee = settable.boolValue
    return error.rawValue
}

@_cdecl("ax_attribute_get_value_count")
public func ax_attribute_get_value_count(
    _ handle: UnsafeMutableRawPointer?,
    _ attribute: UnsafePointer<CChar>?,
    _ outCount: UnsafeMutablePointer<Int>?
) -> Int32 {
    guard let handle, let attribute, let outCount else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    var count = CFIndex(0)
    let error = AXUIElementGetAttributeValueCount(element, String(cString: attribute) as CFString, &count)
    outCount.pointee = count
    return error.rawValue
}

@_cdecl("ax_attribute_copy_value")
public func ax_attribute_copy_value(
    _ handle: UnsafeMutableRawPointer?,
    _ attribute: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let handle, let attribute, let outValue else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    var value: CFTypeRef?
    let error = AXUIElementCopyAttributeValue(element, String(cString: attribute) as CFString, &value)
    if error == .success, let value {
        outValue.pointee = retainObject(value)
    } else {
        outValue.pointee = nil
    }
    return error.rawValue
}

@_cdecl("ax_attribute_copy_values")
public func ax_attribute_copy_values(
    _ handle: UnsafeMutableRawPointer?,
    _ attribute: UnsafePointer<CChar>?,
    _ index: Int,
    _ maxValues: Int,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let handle, let attribute, let outValue else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    var values: CFArray?
    let error = AXUIElementCopyAttributeValues(
        element,
        String(cString: attribute) as CFString,
        index,
        maxValues,
        &values)
    if error == .success, let values {
        outValue.pointee = retainObject(values)
    } else {
        outValue.pointee = nil
    }
    return error.rawValue
}

@_cdecl("ax_attribute_set_value")
public func ax_attribute_set_value(
    _ handle: UnsafeMutableRawPointer?,
    _ attribute: UnsafePointer<CChar>?,
    _ valueHandle: UnsafeMutableRawPointer?
) -> Int32 {
    guard let handle, let attribute, let valueHandle else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    let value: CFTypeRef = unretainedObject(valueHandle)
    return AXUIElementSetAttributeValue(element, String(cString: attribute) as CFString, value).rawValue
}

@_cdecl("ax_attribute_copy_multiple_values")
public func ax_attribute_copy_multiple_values(
    _ handle: UnsafeMutableRawPointer?,
    _ attributes: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int,
    _ options: UInt32,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let handle, let attributes, let outValue, count >= 0 else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    let names = (0..<count).compactMap { attributes[$0].map { String(cString: $0) as CFString } } as CFArray
    var values: CFArray?
    let error = AXUIElementCopyMultipleAttributeValues(
        element,
        names,
        AXCopyMultipleAttributeOptions(rawValue: options),
        &values)
    if error == .success, let values {
        outValue.pointee = retainObject(values)
    } else {
        outValue.pointee = nil
    }
    return error.rawValue
}

@_cdecl("ax_attribute_copy_parameterized_names")
public func ax_attribute_copy_parameterized_names(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let handle else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    var names: CFArray?
    return copyNamesArray(AXUIElementCopyParameterizedAttributeNames(element, &names), names, outValue)
}

@_cdecl("ax_attribute_copy_parameterized_value")
public func ax_attribute_copy_parameterized_value(
    _ handle: UnsafeMutableRawPointer?,
    _ attribute: UnsafePointer<CChar>?,
    _ parameterHandle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let handle, let attribute, let parameterHandle, let outValue else {
        return AXError.illegalArgument.rawValue
    }
    let element: AXUIElement = unretainedObject(handle)
    let parameter: CFTypeRef = unretainedObject(parameterHandle)
    var value: CFTypeRef?
    let error = AXUIElementCopyParameterizedAttributeValue(
        element,
        String(cString: attribute) as CFString,
        parameter,
        &value)
    if error == .success, let value {
        outValue.pointee = retainObject(value)
    } else {
        outValue.pointee = nil
    }
    return error.rawValue
}
