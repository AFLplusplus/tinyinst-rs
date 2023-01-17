use std::{
    env,
    path::Path,
    process::{exit, Command},
};

use cmake::Config;
use git2::Repository;
use which::which;

// TODO: make a way to use official tinyinst
const TINYINST_URL: &str = "https://github.com/elbiazo/TinyInst.git";

fn build_dep_check(tools: &[&str]) {
    for tool in tools {
        which(tool).unwrap_or_else(|_| panic!("Build tool {tool} not found"));
    }
}
fn main() {
    // First we generate .cc and .h files from ffi.rs
    if !cfg!(windows) {
        println!("cargo:warning=No MacOS support yet.");
        exit(0);
    }

    build_dep_check(&["git", "python", "cxxbridge"]);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir_path = Path::new(&out_dir);
    let tinyinst_path = out_dir_path.join("Tinyinst");
    // Clone
    println!("cargo:warning=Pulling TinyInst from github");
    let tinyinst_repo = match Repository::clone(TINYINST_URL, &tinyinst_path) {
        Ok(repo) => repo,
        _ => Repository::open(&tinyinst_path).expect("Failed to open repository"),
    };
    let mut submodules = tinyinst_repo.submodules().unwrap();

    // do git submodule --init --recursive on Tinyinst
    for submodule in &mut submodules {
        submodule.update(true, None).unwrap();
    }

    println!("cargo:warning=Generating Bridge files.");
    copy_tinyinst_files(&tinyinst_path);

    let dst = Config::new(&tinyinst_path)
        .generator("Visual Studio 17 2022") // make this configurable from env variable
        .build_target("tinyinst")
        .profile("Release") // without this, it goes into RelWithDbInfo folder??
        .build();

    println!("cargo:warning={}", dst.display());
    println!("cargo:rustc-link-search={}\\build\\Release", dst.display()); // debug build?
    println!(
        "cargo:rustc-link-search={}\\build\\third_party\\obj\\wkit\\lib",
        dst.display()
    ); //xed

    println!("cargo:rustc-link-lib=static=tinyinst");
    println!("cargo:rustc-link-lib=static=xed");
    println!("cargo:rustc-link-lib=dylib=dbghelp");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=src/tinyinst.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Tinyinst/litecov.cpp");
}

fn copy_tinyinst_files(tinyinst_path: &Path) {
    // source
    Command::new("cxxbridge")
        .args(["src/tinyinst.rs", "-o"])
        .arg(format!("{}/bridge.cc", tinyinst_path.to_string_lossy()))
        .status()
        .unwrap();

    // header
    Command::new("cxxbridge")
        .args(["src/tinyinst.rs", "--header", "-o"])
        .arg(format!("{}/bridge.h", tinyinst_path.to_string_lossy()))
        .status()
        .unwrap();

    // cxx
    Command::new("cxxbridge")
        .args(["--header", "-o"])
        .arg(format!("{}/cxx.h", tinyinst_path.to_string_lossy()))
        .status()
        .unwrap();

    // shim
    std::fs::copy("./src/shim.cc", tinyinst_path.join("shim.cc")).unwrap();
    std::fs::copy("./src/shim.h", tinyinst_path.join("shim.h")).unwrap();

    // runresult
    std::fs::copy("./src/runresult.h", tinyinst_path.join("runresult.h")).unwrap();

    // instrumentation
    std::fs::copy(
        "./src/instrumentation.cpp",
        tinyinst_path.join("instrumentation.cpp"),
    )
    .unwrap();
    std::fs::copy(
        "./src/instrumentation.h",
        tinyinst_path.join("instrumentation.h"),
    )
    .unwrap();

    // tinyinstinstrumentation
    std::fs::copy(
        "./src/tinyinstinstrumentation.cpp",
        tinyinst_path.join("tinyinstinstrumentation.cpp"),
    )
    .unwrap();
    std::fs::copy(
        "./src/tinyinstinstrumentation.h",
        tinyinst_path.join("tinyinstinstrumentation.h"),
    )
    .unwrap();

    // aflcov
    std::fs::copy("./src/aflcov.cpp", tinyinst_path.join("aflcov.cpp")).unwrap();
    std::fs::copy("./src/aflcov.h", tinyinst_path.join("aflcov.h")).unwrap();
}
