import ApplicationServices
import Foundation

private func errorName(for code: Int32) -> String {
    switch AXError(rawValue: code) {
    case .success:
        return "success"
    case .failure:
        return "failure"
    case .illegalArgument:
        return "illegal argument"
    case .invalidUIElement:
        return "invalid UI element"
    case .invalidUIElementObserver:
        return "invalid UI element observer"
    case .cannotComplete:
        return "cannot complete"
    case .attributeUnsupported:
        return "attribute unsupported"
    case .actionUnsupported:
        return "action unsupported"
    case .notificationUnsupported:
        return "notification unsupported"
    case .notImplemented:
        return "not implemented"
    case .notificationAlreadyRegistered:
        return "notification already registered"
    case .notificationNotRegistered:
        return "notification not registered"
    case .apiDisabled:
        return "api disabled"
    case .noValue:
        return "no value"
    case .parameterizedAttributeUnsupported:
        return "parameterized attribute unsupported"
    case .notEnoughPrecision:
        return "not enough precision"
    case nil:
        return "unknown ax error"
    @unknown default:
        return "unknown ax error"
    }
}

@_cdecl("ax_error_copy_description")
public func ax_error_copy_description(_ code: Int32) -> UnsafeMutableRawPointer {
    retainObject(errorName(for: code) as NSString)
}
