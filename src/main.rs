use std::fs;

mod lexer;
//                                                              May the cat rest in peace
const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"), "-Simson");
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

struct Args {
    out_file: Option<String>,
    source: Vec<String>,
    static_link: bool,
    show_stages: bool,
    header: bool,
    cpp_args: Vec<String>
}


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

fn print_help() {
    println!("CUTE Compiler {}", VERSION);
    println!();
    println!("Usage:");
    println!("    cutec [options] <source.cute> [-- <cpp-compiler-arguments...>]");
    println!();
    println!("Options:");
    println!("    -o <file>    Write compiled output to <file>");
    println!("    -s           Statically link the compiled output");
    println!("    -S           Display intermediate compilation stages");
    println!("    -h           Generate a C++ header instead of a C++ source file");
    println!("    -v           Display compiler version information");
    println!("    --version    Display compiler version information");
    println!("    --help       Display this help message");
    println!();
    println!("C++ compiler arguments:");
    println!("    Arguments following '--' are passed directly to the C++ compiler.");
    println!();
    println!("Examples:");
    println!("    cutec src/main.cute");
    println!("    cutec -o program src/main.cute");
    println!("    cutec -s -o program src/main.cute -- -O2 -Wall");
    println!("    cutec -S src/main.cute");
}

fn parse_args(args: &[String]) -> Result<Args, String> {
    let mut parsed = Args {
        out_file: None,
        source: Vec::new(),
        static_link: false,
        show_stages: false,
        header: false,
        cpp_args: Vec::new(),
    };

    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];

        if arg == "--" {
            parsed.cpp_args.extend_from_slice(&args[i + 1..]);
            break;
        }

        match arg.as_str() {
            "-v" | "--version" => {
                print_version();
                std::process::exit(0);
            }

            "--help" => {
                print_help();
            }

            "-o" => {
                i += 1;

                if i >= args.len() {
                    return Err("Expected a filename after '-o'".into());
                }

                parsed.out_file = Some(args[i].clone());
            }

            "-s" => parsed.static_link = true,
            "-S" => parsed.show_stages = true,
            "-h" => parsed.header = true,

            _ if arg.starts_with('-') => {
                return Err(format!("Unknown option: {}", arg));
            }

            _ => parsed.source.push(arg.clone()),
        }

        i += 1;
    }

    if parsed.source.is_empty() {
        if parsed.header {
            return Err("Argument -h is used for generating header files. Use '--help' for help.".into());
        }
        return Err("No CUTE source file specified".into());
    }

    Ok(parsed)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let args = match parse_args(&args) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("cutec: {}", err);
            std::process::exit(1);
        }
    };

    let code = fs::read_to_string(args.source[0].clone())
        .expect("cutec: Failed to read file");

    let tokens = lexer::tokenize(code.clone());
    for tok in tokens.clone() {
        println!("{}:{}\t {}", tok.line, tok.column, tok.lex);
    }
}
