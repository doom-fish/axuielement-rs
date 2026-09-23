import ApplicationServices
import AXUIElementLegacyAPI
import Foundation

@_cdecl("ax_process_trust_api_enabled")
public func ax_process_trust_api_enabled() -> Bool {
    AXLegacyAPIEnabled()
}

@_cdecl("ax_process_trust_is_trusted")
public func ax_process_trust_is_trusted() -> Bool {
    AXIsProcessTrusted()
}

@_cdecl("ax_process_trust_is_trusted_with_prompt")
public func ax_process_trust_is_trusted_with_prompt(_ prompt: Bool) -> Bool {
    let key = kAXTrustedCheckOptionPrompt.takeUnretainedValue() as String
    let options = [key: prompt] as NSDictionary
    return AXIsProcessTrustedWithOptions(options)
}

@_cdecl("ax_process_trust_make_process_trusted")
public func ax_process_trust_make_process_trusted(_ executablePath: UnsafePointer<CChar>?) -> Int32 {
    guard let executablePath, let string = String(validatingUTF8: executablePath) else {
        return AXError.illegalArgument.rawValue
    }
    return AXLegacyMakeProcessTrusted(string as CFString).rawValue
}
