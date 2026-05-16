// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "AXUIElementBridge",
    platforms: [
        .macOS(.v10_13)
    ],
    products: [
        .library(
            name: "AXUIElementBridge",
            type: .static,
            targets: ["AXUIElementBridge"])
    ],
    targets: [
        .target(
            name: "AXUIElementBridge",
            path: "Sources/AXUIElementBridge")
    ]
)
