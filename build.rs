use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn run(cmd: &mut Command) {
    assert!(cmd.status().unwrap().success(), "{cmd:?} failed");
}

fn configure(src: &str, build: &str) -> Command {
    let mut cmd = Command::new("cmake");
    cmd.args([
        "-S",
        src,
        "-B",
        build,
        "-DCMAKE_BUILD_TYPE=Release",
        // rustc links position-independent binaries, so the static archives it
        // pulls in have to be position-independent too.
        "-DCMAKE_POSITION_INDEPENDENT_CODE=ON",
    ]);
    cmd
}

fn main() {
    let src = format!("{}/hoshidicts", env::var("CARGO_MANIFEST_DIR").unwrap());
    let build = format!("{}/build", env::var("OUT_DIR").unwrap());
    let msvc = env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc";

    assert!(
        Path::new(&src).join("CMakeLists.txt").is_file(),
        "hoshidicts is empty - run `git submodule update --init --recursive`"
    );

    if !configure(&src, &build).status().unwrap().success() {
        fs::remove_dir_all(&build).ok();
        run(&mut configure(&src, &build));
    }
    // Multi-config generators ignore CMAKE_BUILD_TYPE and need --config.
    run(Command::new("cmake").args([
        "--build",
        &build,
        "--target",
        "hoshidicts",
        "--parallel",
        "--config",
        "Release",
    ]));

    for dir in [
        "",
        "external/utf8proc",
        "external/libdeflate",
        "external/zstd/build/cmake/lib",
    ] {
        println!("cargo:rustc-link-search=native={build}/{dir}");
        // Multi-config generators write archives to a per-config subdirectory.
        let config_dir = format!("{build}/{dir}/Release");
        if Path::new(&config_dir).is_dir() {
            println!("cargo:rustc-link-search=native={config_dir}");
        }
    }

    // MSVC renames the static archives so they cannot clash with import libraries.
    let (utf8proc, deflate, zstd) = match msvc {
        true => ("utf8proc_static", "deflatestatic", "zstd_static"),
        false => ("utf8proc", "deflate", "zstd"),
    };
    for lib in ["hoshidicts", utf8proc, deflate, zstd] {
        println!("cargo:rustc-link-lib=static={lib}");
    }

    // MSVC links the C++ runtime on its own.
    if !msvc {
        println!(
            "cargo:rustc-link-lib={}",
            match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
                "linux" | "android" => "stdc++",
                _ => "c++",
            }
        );
    }

    println!("cargo:rerun-if-changed=hoshidicts/src");
    println!("cargo:rerun-if-changed=hoshidicts/include");
}
