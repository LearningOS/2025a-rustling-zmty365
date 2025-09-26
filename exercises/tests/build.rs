//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 直接使用正确的格式输出 - 删除所有模板文本
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // 直接使用正确的格式输出 - 删除所有模板文本
    println!("cargo:rustc-cfg=feature=\"pass\"");
    
    // 可选：重新运行构建如果构建脚本变化
    println!("cargo:rerun-if-changed=build.rs");
}