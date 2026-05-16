use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=swift-bridge");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-env-changed=DEVELOPER_DIR");
    println!("cargo:rerun-if-env-changed=SDKROOT");

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR");
    let swift_build_dir = format!("{out_dir}/swift-build");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let swift_triple = match target_arch.as_str() {
        "x86_64" => "x86_64-apple-macosx",
        "aarch64" => "arm64-apple-macosx",
        other => panic!("unsupported target arch: {other}"),
    };

    let output = Command::new("swift")
        .args([
            "build",
            "-c",
            "release",
            "--triple",
            swift_triple,
            "--package-path",
            "swift-bridge",
            "--scratch-path",
            &swift_build_dir,
        ])
        .output()
        .expect("failed to build Swift bridge");

    if !output.status.success() {
        eprintln!(
            "Swift build stdout:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        eprintln!(
            "Swift build stderr:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!("Swift build failed with status {:?}", output.status.code());
    }

    println!("cargo:rustc-link-search=native={swift_build_dir}/release");
    println!("cargo:rustc-link-lib=static=AXUIElementBridge");
    for framework in [
        "ApplicationServices",
        "CoreFoundation",
        "CoreGraphics",
        "Foundation",
    ] {
        println!("cargo:rustc-link-lib=framework={framework}");
    }
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    if let Ok(output) = Command::new("xcode-select").arg("-p").output() {
        if output.status.success() {
            let xcode_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let old_path = format!(
                "{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift-5.5/macosx"
            );
            let new_path =
                format!("{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
            println!("cargo:rustc-link-search=native={old_path}");
            println!("cargo:rustc-link-search=native={new_path}");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{old_path}");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{new_path}");
        }
    }
}
