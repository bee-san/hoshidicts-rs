use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn run(cmd: &mut Command) {
    assert!(cmd.status().unwrap().success(), "{cmd:?} failed");
}

fn cxx_runtime_dir(build: &str, runtime: &str) -> Option<String> {
    let cache = fs::read_to_string(Path::new(build).join("CMakeCache.txt")).ok()?;
    let compiler = cache
        .lines()
        .find_map(|line| line.strip_prefix("CMAKE_CXX_COMPILER:FILEPATH="))?;
    let output = Command::new(compiler)
        .arg(format!("-print-file-name=lib{runtime}.so"))
        .output()
        .ok()?;
    let path = String::from_utf8(output.stdout).ok()?;
    let path = Path::new(path.trim());
    match path.is_absolute() {
        true => Some(path.parent()?.to_str()?.to_owned()),
        false => None,
    }
}

fn configure(src: &str, build: &str, msvc: bool) -> Command {
    let mut cmd = Command::new("cmake");
    cmd.args([
        "-S",
        src,
        "-B",
        build,
        "-DCMAKE_BUILD_TYPE=Release",
        "-DCMAKE_POSITION_INDEPENDENT_CODE=ON",
    ]);
    if msvc {
        cmd.arg("-DCMAKE_CXX_FLAGS=/Zc:__cplusplus");
    }
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

    if !configure(&src, &build, msvc).status().unwrap().success() {
        fs::remove_dir_all(&build).ok();
        run(&mut configure(&src, &build, msvc));
    }
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
        let config_dir = format!("{build}/{dir}/Release");
        if Path::new(&config_dir).is_dir() {
            println!("cargo:rustc-link-search=native={config_dir}");
        }
    }

    let (utf8proc, deflate, zstd) = match msvc {
        true => ("utf8proc_static", "deflatestatic", "zstd_static"),
        false => ("utf8proc", "deflate", "zstd"),
    };
    for lib in ["hoshidicts", utf8proc, deflate, zstd] {
        println!("cargo:rustc-link-lib=static={lib}");
    }

    if !msvc {
        let runtime = match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
            "linux" | "android" => "stdc++",
            _ => "c++",
        };
        if let Some(dir) = cxx_runtime_dir(&build, runtime) {
            println!("cargo:rustc-link-search=native={dir}");
        }
        println!("cargo:rustc-link-lib={runtime}");
    }

    println!("cargo:rerun-if-changed=hoshidicts/src");
    println!("cargo:rerun-if-changed=hoshidicts/include");
}
