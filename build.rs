use std::env;
use std::path::Path;
use std::process::Command;

fn run(cmd: &mut Command) {
    assert!(cmd.status().unwrap().success(), "{cmd:?} failed");
}

fn main() {
    assert!(
        Path::new("hoshidicts/CMakeLists.txt").is_file(),
        "hoshidicts is empty - run `git submodule update --init --recursive`"
    );

    let build = format!("{}/build", env::var("OUT_DIR").unwrap());

    run(Command::new("cmake").args([
        "-S",
        "hoshidicts",
        "-B",
        &build,
        "-DCMAKE_BUILD_TYPE=Release",
    ]));
    run(Command::new("cmake").args(["--build", &build, "--target", "hoshidicts", "--parallel"]));

    for dir in [
        "",
        "external/utf8proc",
        "external/libdeflate",
        "external/zstd/build/cmake/lib",
    ] {
        println!("cargo:rustc-link-search=native={build}/{dir}");
    }
    for lib in ["hoshidicts", "utf8proc", "deflate", "zstd"] {
        println!("cargo:rustc-link-lib=static={lib}");
    }
    println!(
        "cargo:rustc-link-lib={}",
        match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
            "linux" => "stdc++",
            _ => "c++",
        }
    );

    println!("cargo:rerun-if-changed=hoshidicts/src");
    println!("cargo:rerun-if-changed=hoshidicts/include");
}
