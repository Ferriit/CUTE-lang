const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
const LICENSE: &str = env!("CARGO_PKG_LICENSE");
const NAME: &str = "cutec";
const TARGET: &str = env!("CUTE_TARGET");
const BUILD_TYPE: &str = env!("CUTE_BUILD_TYPE");
const COMMIT: &str = env!("CUTE_COMMIT");

#[cfg(target_arch = "x86_64")]
const ARCH: &str = "x86_64";
#[cfg(target_arch = "aarch64")]
const ARCH: &str = "aarch64";
#[cfg(target_arch = "x86")]
const ARCH: &str = "x86";


fn print_version() {
    println!("{} {}", NAME, VERSION);
    println!("Copyright (c) 2026 CUTE contributors");
    println!("License: {}", LICENSE);

    println!("Build info:");
    println!("\tTarget:\t\t{}", TARGET);
    println!("\tArch:\t\t{}", ARCH);
    println!("\tBuild type:\t{}", BUILD_TYPE);
    println!("\tCommit:\t\t{}", COMMIT);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    print_version();
}
