use std::process::Command;

fn main() {
    println!("cargo:rustc-env=CUTE_TARGET={}", std::env::var("TARGET").unwrap());
    println!("cargo:rustc-env=CUTE_BUILD_TYPE={}", std::env::var("PROFILE").unwrap());

    println!("cargo:rerun-if-changed=.git/HEAD");

    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_else(|| "unknown".to_owned());

    println!("cargo:rustc-env=CUTE_COMMIT={}", commit);
}
