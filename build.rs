use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn run(cmd: &mut Command) {
    assert!(cmd.status().unwrap().success(), "{cmd:?} failed");
}

fn configure(src: &str, build: &str) -> Command {
    let mut cmd = Command::new("cmake");
    cmd.args(["-S", src, "-B", build, "-DCMAKE_BUILD_TYPE=Release"]);
    cmd
}

fn main() {
    let src = format!("{}/hoshidicts", env::var("CARGO_MANIFEST_DIR").unwrap());
    let build = format!("{}/build", env::var("OUT_DIR").unwrap());

    assert!(
        Path::new(&src).join("CMakeLists.txt").is_file(),
        "hoshidicts is empty - run `git submodule update --init --recursive`"
    );

    if !configure(&src, &build).status().unwrap().success() {
        fs::remove_dir_all(&build).ok();
        run(&mut configure(&src, &build));
    }
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
