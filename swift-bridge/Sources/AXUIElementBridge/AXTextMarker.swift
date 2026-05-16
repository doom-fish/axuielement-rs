import ApplicationServices
import Foundation

@_cdecl("ax_text_marker_get_type_id")
public func ax_text_marker_get_type_id() -> UInt {
    AXTextMarkerGetTypeID()
}

@_cdecl("ax_text_marker_create")
public func ax_text_marker_create(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int
) -> UnsafeMutableRawPointer? {
    guard let bytes, length >= 0 else {
        return nil
    }
    let marker = AXTextMarkerCreate(kCFAllocatorDefault, bytes, length)
    return retainObject(marker)
}

@_cdecl("ax_text_marker_retain")
public func ax_text_marker_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let marker: AXTextMarker = unretainedObject(handle)
    return retainObject(marker)
}

@_cdecl("ax_text_marker_release")
public func ax_text_marker_release(_ handle: UnsafeMutableRawPointer?) {
    releaseObject(handle)
}

@_cdecl("ax_text_marker_len")
public func ax_text_marker_len(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle else {
        return 0
    }
    let marker: AXTextMarker = unretainedObject(handle)
    return AXTextMarkerGetLength(marker)
}

@_cdecl("ax_text_marker_copy_bytes")
public func ax_text_marker_copy_bytes(
    _ handle: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UInt8>?,
    _ capacity: Int
) -> Bool {
    guard let handle, let buffer, capacity >= 0 else {
        return false
    }
    let marker: AXTextMarker = unretainedObject(handle)
    let length = AXTextMarkerGetLength(marker)
    guard capacity >= length else {
        return false
    }
    let bytes = AXTextMarkerGetBytePtr(marker)
    buffer.initialize(from: bytes, count: length)
    return true
}

@_cdecl("ax_text_marker_range_get_type_id")
public func ax_text_marker_range_get_type_id() -> UInt {
    AXTextMarkerRangeGetTypeID()
}

@_cdecl("ax_text_marker_range_create")
public func ax_text_marker_range_create(
    _ startHandle: UnsafeMutableRawPointer?,
    _ endHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let startHandle, let endHandle else {
        return nil
    }
    let start: AXTextMarker = unretainedObject(startHandle)
    let end: AXTextMarker = unretainedObject(endHandle)
    let range = AXTextMarkerRangeCreate(kCFAllocatorDefault, start, end)
    return retainObject(range)
}

@_cdecl("ax_text_marker_range_create_with_bytes")
public func ax_text_marker_range_create_with_bytes(
    _ startBytes: UnsafePointer<UInt8>?,
    _ startLength: Int,
    _ endBytes: UnsafePointer<UInt8>?,
    _ endLength: Int
) -> UnsafeMutableRawPointer? {
    guard let startBytes, let endBytes, startLength >= 0, endLength >= 0 else {
        return nil
    }
    let range = AXTextMarkerRangeCreateWithBytes(
        kCFAllocatorDefault,
        startBytes,
        startLength,
        endBytes,
        endLength)
    return retainObject(range)
}

@_cdecl("ax_text_marker_range_retain")
public func ax_text_marker_range_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let range: AXTextMarkerRange = unretainedObject(handle)
    return retainObject(range)
}

@_cdecl("ax_text_marker_range_release")
public func ax_text_marker_range_release(_ handle: UnsafeMutableRawPointer?) {
    releaseObject(handle)
}

@_cdecl("ax_text_marker_range_copy_start_marker")
public func ax_text_marker_range_copy_start_marker(
    _ handle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let range: AXTextMarkerRange = unretainedObject(handle)
    return retainObject(AXTextMarkerRangeCopyStartMarker(range))
}

@_cdecl("ax_text_marker_range_copy_end_marker")
public func ax_text_marker_range_copy_end_marker(
    _ handle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let range: AXTextMarkerRange = unretainedObject(handle)
    return retainObject(AXTextMarkerRangeCopyEndMarker(range))
}
