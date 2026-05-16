import ApplicationServices
import CoreFoundation
import Foundation

@inline(__always)
func retainObject(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func unretainedObject<T: AnyObject>(_ handle: UnsafeMutableRawPointer?) -> T {
    Unmanaged<T>.fromOpaque(handle!).takeUnretainedValue()
}

@inline(__always)
func releaseObject(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@inline(__always)
func stringFromCString(_ pointer: UnsafePointer<CChar>?) -> String? {
    guard let pointer else {
        return nil
    }
    return String(cString: pointer)
}

@inline(__always)
func copyUTF8(_ string: String, _ buffer: UnsafeMutablePointer<CChar>?, _ capacity: Int) -> Bool {
    guard let buffer, capacity > 0 else {
        return false
    }
    return string.withCString { source in
        let length = Int(strlen(source))
        guard length + 1 <= capacity else {
            return false
        }
        buffer.initialize(from: source, count: length)
        buffer[length] = 0
        return true
    }
}

private enum AXBridgeValueKind: UInt32 {
    case null = 0
    case string = 1
    case bool = 2
    case integer = 3
    case double = 4
    case element = 5
    case array = 6
    case point = 7
    case size = 8
    case rect = 9
    case range = 10
    case error = 11
    case textMarker = 12
    case textMarkerRange = 13
    case data = 14
    case dictionary = 15
    case attributedString = 16
    case url = 17
    case unknown = 255
}

func cfTypeID(_ object: AnyObject) -> CFTypeID {
    CFGetTypeID(object)
}

private func numberKind(_ number: NSNumber) -> AXBridgeValueKind {
    if cfTypeID(number) == CFBooleanGetTypeID() {
        return .bool
    }
    return CFNumberIsFloatType(number) ? .double : .integer
}

private func stringObject(from object: AnyObject) -> NSString? {
    if let string = object as? NSString {
        return string
    }
    if let attributed = object as? NSAttributedString {
        return attributed.string as NSString
    }
    if let url = object as? NSURL, let absoluteString = url.absoluteString {
        return absoluteString as NSString
    }
    return nil
}

private func valueKind(for object: AnyObject) -> AXBridgeValueKind {
    let typeID = cfTypeID(object)
    if object is NSNull {
        return .null
    }
    if typeID == AXValueGetTypeID() {
        let value = unsafeBitCast(object, to: AXValue.self)
        switch AXValueGetType(value) {
        case .cgPoint:
            return .point
        case .cgSize:
            return .size
        case .cgRect:
            return .rect
        case .cfRange:
            return .range
        case .axError:
            return .error
        case .illegal:
            return .unknown
        @unknown default:
            return .unknown
        }
    }
    if typeID == AXUIElementGetTypeID() {
        return .element
    }
    if typeID == AXTextMarkerGetTypeID() {
        return .textMarker
    }
    if typeID == AXTextMarkerRangeGetTypeID() {
        return .textMarkerRange
    }
    if object is NSArray {
        return .array
    }
    if object is NSDictionary {
        return .dictionary
    }
    if object is NSData {
        return .data
    }
    if object is NSAttributedString {
        return .attributedString
    }
    if object is NSURL {
        return .url
    }
    if object is NSString {
        return .string
    }
    if let number = object as? NSNumber {
        return numberKind(number)
    }
    return .unknown
}

private func dictionaryKeys(_ dictionary: NSDictionary) -> [String] {
    dictionary.allKeys.compactMap { ($0 as? NSString).map(String.init) }.sorted()
}

@_cdecl("ax_string_retain")
public func ax_string_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let string: NSString = unretainedObject(handle)
    return retainObject(string)
}

@_cdecl("ax_string_release")
public func ax_string_release(_ handle: UnsafeMutableRawPointer?) {
    releaseObject(handle)
}

@_cdecl("ax_string_len")
public func ax_string_len(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle else {
        return 0
    }
    let string: NSString = unretainedObject(handle)
    return string.lengthOfBytes(using: String.Encoding.utf8.rawValue)
}

@_cdecl("ax_string_copy_utf8")
public func ax_string_copy_utf8(
    _ handle: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<CChar>?,
    _ capacity: Int
) -> Bool {
    guard let handle else {
        return false
    }
    let string: NSString = unretainedObject(handle)
    return copyUTF8(string as String, buffer, capacity)
}

@_cdecl("ax_value_get_type_id")
public func ax_value_get_type_id() -> UInt {
    AXValueGetTypeID()
}

@_cdecl("ax_value_retain")
public func ax_value_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let object: AnyObject = unretainedObject(handle)
    return retainObject(object)
}

@_cdecl("ax_value_release")
public func ax_value_release(_ handle: UnsafeMutableRawPointer?) {
    releaseObject(handle)
}

@_cdecl("ax_value_kind")
public func ax_value_kind(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else {
        return AXBridgeValueKind.null.rawValue
    }
    let object: AnyObject = unretainedObject(handle)
    return valueKind(for: object).rawValue
}

@_cdecl("ax_value_copy_string")
public func ax_value_copy_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let object: AnyObject = unretainedObject(handle)
    guard let string = stringObject(from: object) else {
        return nil
    }
    return retainObject(string)
}

@_cdecl("ax_value_get_bool")
public func ax_value_get_bool(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Bool>?
) -> Bool {
    guard let handle, let outValue else {
        return false
    }
    let object: AnyObject = unretainedObject(handle)
    guard let number = object as? NSNumber, numberKind(number) == .bool else {
        return false
    }
    outValue.pointee = number.boolValue
    return true
}

@_cdecl("ax_value_get_i64")
public func ax_value_get_i64(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int64>?
) -> Bool {
    guard let handle, let outValue else {
        return false
    }
    let object: AnyObject = unretainedObject(handle)
    guard let number = object as? NSNumber, numberKind(number) == .integer else {
        return false
    }
    outValue.pointee = number.int64Value
    return true
}

@_cdecl("ax_value_get_f64")
public func ax_value_get_f64(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Double>?
) -> Bool {
    guard let handle, let outValue else {
        return false
    }
    let object: AnyObject = unretainedObject(handle)
    guard let number = object as? NSNumber else {
        return false
    }
    switch numberKind(number) {
    case .integer, .double:
        outValue.pointee = number.doubleValue
        return true
    default:
        return false
    }
}

@_cdecl("ax_value_get_point")
public func ax_value_get_point(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<CGPoint>?
) -> Bool {
    guard let handle, let outValue else {
        return false
    }
    let object: AnyObject = unretainedObject(handle)
    guard cfTypeID(object) == AXValueGetTypeID() else {
        return false
    }
    let value = unsafeBitCast(object, to: AXValue.self)
    guard AXValueGetType(value) == .cgPoint else {
        return false
    }
    return AXValueGetValue(value, .cgPoint, outValue)
}

@_cdecl("ax_value_get_size")
public func ax_value_get_size(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<CGSize>?
) -> Bool {
    guard let handle, let outValue else {
        return false
    }
    let object: AnyObject = unretainedObject(handle)
    guard cfTypeID(object) == AXValueGetTypeID() else {
        return false
    }
    let value = unsafeBitCast(object, to: AXValue.self)
    guard AXValueGetType(value) == .cgSize else {
        return false
    }
    return AXValueGetValue(value, .cgSize, outValue)
}

@_cdecl("ax_value_get_rect")
public func ax_value_get_rect(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<CGRect>?
) -> Bool {
    guard let handle, let outValue else {
        return false
    }
    let object: AnyObject = unretainedObject(handle)
    guard cfTypeID(object) == AXValueGetTypeID() else {
        return false
    }
    let value = unsafeBitCast(object, to: AXValue.self)
    guard AXValueGetType(value) == .cgRect else {
        return false
    }
    return AXValueGetValue(value, .cgRect, outValue)
}

@_cdecl("ax_value_get_range")
public func ax_value_get_range(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<CFRange>?
) -> Bool {
    guard let handle, let outValue else {
        return false
    }
    let object: AnyObject = unretainedObject(handle)
    guard cfTypeID(object) == AXValueGetTypeID() else {
        return false
    }
    let value = unsafeBitCast(object, to: AXValue.self)
    guard AXValueGetType(value) == .cfRange else {
        return false
    }
    return AXValueGetValue(value, .cfRange, outValue)
}

@_cdecl("ax_value_get_error_code")
public func ax_value_get_error_code(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?
) -> Bool {
    guard let handle, let outValue else {
        return false
    }
    let object: AnyObject = unretainedObject(handle)
    guard cfTypeID(object) == AXValueGetTypeID() else {
        return false
    }
    let value = unsafeBitCast(object, to: AXValue.self)
    guard AXValueGetType(value) == .axError else {
        return false
    }
    var error = AXError.failure
    guard AXValueGetValue(value, .axError, &error) else {
        return false
    }
    outValue.pointee = error.rawValue
    return true
}

@_cdecl("ax_value_copy_element")
public func ax_value_copy_element(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let object: AnyObject = unretainedObject(handle)
    guard cfTypeID(object) == AXUIElementGetTypeID() else {
        return nil
    }
    let element = unsafeBitCast(object, to: AXUIElement.self)
    return retainObject(element)
}

@_cdecl("ax_value_array_len")
public func ax_value_array_len(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle else {
        return 0
    }
    let object: AnyObject = unretainedObject(handle)
    guard let array = object as? NSArray else {
        return 0
    }
    return array.count
}

@_cdecl("ax_value_array_get")
public func ax_value_array_get(
    _ handle: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let object: AnyObject = unretainedObject(handle)
    guard let array = object as? NSArray, index >= 0, index < array.count else {
        return nil
    }
    let value = array.object(at: index) as AnyObject
    return retainObject(value)
}

@_cdecl("ax_value_dictionary_len")
public func ax_value_dictionary_len(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle else {
        return 0
    }
    let object: AnyObject = unretainedObject(handle)
    guard let dictionary = object as? NSDictionary else {
        return 0
    }
    return dictionaryKeys(dictionary).count
}

@_cdecl("ax_value_dictionary_key_at")
public func ax_value_dictionary_key_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let object: AnyObject = unretainedObject(handle)
    guard let dictionary = object as? NSDictionary else {
        return nil
    }
    let keys = dictionaryKeys(dictionary)
    guard index >= 0, index < keys.count else {
        return nil
    }
    return retainObject(keys[index] as NSString)
}

@_cdecl("ax_value_dictionary_value_at")
public func ax_value_dictionary_value_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let object: AnyObject = unretainedObject(handle)
    guard let dictionary = object as? NSDictionary else {
        return nil
    }
    let keys = dictionaryKeys(dictionary)
    guard index >= 0, index < keys.count else {
        return nil
    }
    let key = keys[index] as NSString
    guard let value = dictionary.object(forKey: key) as AnyObject? else {
        return nil
    }
    return retainObject(value)
}

@_cdecl("ax_value_data_len")
public func ax_value_data_len(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle else {
        return 0
    }
    let object: AnyObject = unretainedObject(handle)
    guard let data = object as? NSData else {
        return 0
    }
    return data.length
}

@_cdecl("ax_value_copy_data_bytes")
public func ax_value_copy_data_bytes(
    _ handle: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UInt8>?,
    _ capacity: Int
) -> Bool {
    guard let handle, let buffer, capacity >= 0 else {
        return false
    }
    let object: AnyObject = unretainedObject(handle)
    guard let data = object as? NSData, capacity >= data.length else {
        return false
    }
    data.getBytes(buffer, length: data.length)
    return true
}

@_cdecl("ax_value_create_null")
public func ax_value_create_null() -> UnsafeMutableRawPointer {
    retainObject(NSNull())
}

@_cdecl("ax_value_create_string")
public func ax_value_create_string(_ value: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let string = stringFromCString(value) else {
        return nil
    }
    return retainObject(string as NSString)
}

@_cdecl("ax_value_create_bool")
public func ax_value_create_bool(_ value: Bool) -> UnsafeMutableRawPointer {
    retainObject(NSNumber(value: value))
}

@_cdecl("ax_value_create_i64")
public func ax_value_create_i64(_ value: Int64) -> UnsafeMutableRawPointer {
    retainObject(NSNumber(value: value))
}

@_cdecl("ax_value_create_f64")
public func ax_value_create_f64(_ value: Double) -> UnsafeMutableRawPointer {
    retainObject(NSNumber(value: value))
}

@_cdecl("ax_value_create_point")
public func ax_value_create_point(_ value: CGPoint) -> UnsafeMutableRawPointer? {
    var copy = value
    guard let wrapped = AXValueCreate(.cgPoint, &copy) else {
        return nil
    }
    return retainObject(wrapped)
}

@_cdecl("ax_value_create_size")
public func ax_value_create_size(_ value: CGSize) -> UnsafeMutableRawPointer? {
    var copy = value
    guard let wrapped = AXValueCreate(.cgSize, &copy) else {
        return nil
    }
    return retainObject(wrapped)
}

@_cdecl("ax_value_create_rect")
public func ax_value_create_rect(_ value: CGRect) -> UnsafeMutableRawPointer? {
    var copy = value
    guard let wrapped = AXValueCreate(.cgRect, &copy) else {
        return nil
    }
    return retainObject(wrapped)
}

@_cdecl("ax_value_create_range")
public func ax_value_create_range(_ value: CFRange) -> UnsafeMutableRawPointer? {
    var copy = value
    guard let wrapped = AXValueCreate(.cfRange, &copy) else {
        return nil
    }
    return retainObject(wrapped)
}

@_cdecl("ax_value_create_error_code")
public func ax_value_create_error_code(_ code: Int32) -> UnsafeMutableRawPointer? {
    var error = AXError(rawValue: code) ?? .failure
    guard let wrapped = AXValueCreate(.axError, &error) else {
        return nil
    }
    return retainObject(wrapped)
}

@_cdecl("ax_value_create_element")
public func ax_value_create_element(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let element: AXUIElement = unretainedObject(handle)
    return retainObject(element)
}

@_cdecl("ax_value_create_text_marker")
public func ax_value_create_text_marker(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let marker: AXTextMarker = unretainedObject(handle)
    return retainObject(marker)
}

@_cdecl("ax_value_create_text_marker_range")
public func ax_value_create_text_marker_range(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let range: AXTextMarkerRange = unretainedObject(handle)
    return retainObject(range)
}

@_cdecl("ax_value_create_data")
public func ax_value_create_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int
) -> UnsafeMutableRawPointer? {
    guard let bytes, length >= 0 else {
        return nil
    }
    let data = Data(bytes: bytes, count: length) as NSData
    return retainObject(data)
}

@_cdecl("ax_value_create_array")
public func ax_value_create_array(
    _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) -> UnsafeMutableRawPointer? {
    guard let values, count >= 0 else {
        return nil
    }
    let array: [AnyObject] = (0..<count).map { index in
        guard let handle = values[index] else {
            return NSNull()
        }
        return unretainedObject(handle) as AnyObject
    }
    return retainObject(array as NSArray)
}

@_cdecl("ax_value_create_dictionary")
public func ax_value_create_dictionary(
    _ keys: UnsafePointer<UnsafePointer<CChar>?>?,
    _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) -> UnsafeMutableRawPointer? {
    guard let keys, let values, count >= 0 else {
        return nil
    }
    let dictionary = NSMutableDictionary(capacity: count)
    for index in 0..<count {
        guard let key = stringFromCString(keys[index]) else {
            continue
        }
        let value: AnyObject = values[index].map { unretainedObject($0) as AnyObject } ?? NSNull()
        dictionary[key] = value
    }
    return retainObject(dictionary)
}
