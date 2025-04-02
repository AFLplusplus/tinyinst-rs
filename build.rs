use std::{env, fs, path::Path, process::Command};

use cmake::Config;
use git2::{Oid, Repository};
use which::which;

const TINYINST_URL: &str = "https://github.com/googleprojectzero/TinyInst.git";
const TINYINST_DIRNAME: &str = "Tinyinst";
const TINYINST_REVISION: &str = "69ae1ff55eac8cb5d2e9a257c5650486ffe2af04";

fn build_dep_check(tools: &[&str]) -> bool {
    for tool in tools {
        let found = which(tool);
        if found.is_err() {
            println!("cargo:warning={tool} not found! Couldn't build tinyinst_rs");
            return false;
        }
    }
    return true;
}

fn main() {
    if !build_dep_check(&["git", "cxxbridge", "cmake"]) {
        return;
    }

    #[cfg(target_os = "windows")]
    let cmake_generator = "Visual Studio 17 2022";
    #[cfg(target_vendor = "apple")]
    let cmake_generator = "Xcode";
    #[cfg(target_os = "linux")]
    let cmake_generator = "Unix Makefiles";

    let custom_tinyinst_generator =
        env::var_os("CUSTOM_TINYINST_GENERATOR").map(|x| x.to_string_lossy().to_string());

    env::set_var("CXXFLAGS", "-std=c++17");

    let tinyinst_generator = if let Some(generator) = custom_tinyinst_generator.as_ref() {
        generator
    } else {
        cmake_generator
    };

    let custum_tinyinst_dir =
        env::var_os("CUSTOM_TINYINST_DIR").map(|x| x.to_string_lossy().to_string());
    let custum_tinyinst_no_build = env::var("CUSTOM_TINYINST_NO_BUILD").is_ok();

    println!("cargo:rerun-if-env-changed=CUSTOM_TINYINST_DIR");
    println!("cargo:rerun-if-env-changed=CUSTOM_TINYINST_NO_BUILD");
    println!("cargo:rerun-if-env-changed=CUSTOM_TINYINST_GENERATOR");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir_path = Path::new(&out_dir);
    let mut target_dir = out_dir_path.to_path_buf();
    target_dir.pop();
    target_dir.pop();
    target_dir.pop();

    let tinyinst_path = if let Some(tinyinst_dir) = custum_tinyinst_dir.as_ref() {
        Path::new(&tinyinst_dir).to_path_buf()
    } else {
        let tinyinst_path = out_dir_path.join(TINYINST_DIRNAME);
        let tinyinst_rev = target_dir.join("TINYINST_REVISION");
        // if revision exists and its different, remove Tinyinst dir
        if tinyinst_rev.exists()
            && fs::read_to_string(&tinyinst_rev).expect("Failed to read TINYINST_REVISION")
                != TINYINST_REVISION
        {
            println!("cargo:warning=Removing Tinyinst dir. Revision changed");
            let _ = fs::remove_dir_all(&tinyinst_path);
        }

        // Check if directory doesn't exist, clone
        if !tinyinst_path.is_dir() {
            println!("cargo:warning=Pulling TinyInst from github");
            let tinyinst_repo = match Repository::clone(TINYINST_URL, &tinyinst_path) {
                Ok(repo) => repo,
                _ => Repository::open(&tinyinst_path).expect("Failed to open repository"),
            };

            // checkout correct commit
            let oid = Oid::from_str(TINYINST_REVISION).unwrap();
            let commit = tinyinst_repo.find_commit(oid).unwrap();

            let _ = tinyinst_repo.branch(TINYINST_REVISION, &commit, false);
            let obj = tinyinst_repo
                .revparse_single(&("refs/heads/".to_owned() + TINYINST_REVISION))
                .unwrap();

            tinyinst_repo.checkout_tree(&obj, None).unwrap();

            tinyinst_repo
                .set_head(&("refs/heads/".to_owned() + TINYINST_REVISION))
                .unwrap();

            let mut submodules = tinyinst_repo.submodules().unwrap();

            // do git submodule update --init --recursive on Tinyinst
            for submodule in &mut submodules {
                submodule.update(true, None).unwrap();
            }

            // write the revision to target dir
            fs::write(&tinyinst_rev, TINYINST_REVISION).unwrap();
        }
        tinyinst_path
    };
    if !custum_tinyinst_no_build {
        println!(
            "cargo:warning=Generating Bridge files. and building for {}",
            &tinyinst_path.to_string_lossy()
        );
        copy_tinyinst_files(&tinyinst_path);

        let _ = Config::new(&tinyinst_path)
            .generator(tinyinst_generator)
            .build_target("tinyinst")
            .profile("Release") // without this, it goes into RelWithDbInfo folder??
            .out_dir(&tinyinst_path)
            .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
            .build();
    }

    // For m1 mac(?)
    println!(
        "cargo:rustc-link-search={}/build/third_party/Release",
        &tinyinst_path.to_string_lossy()
    );

    #[cfg(not(target_os = "linux"))]
    println!(
        "cargo:rustc-link-search={}/build/Release",
        &tinyinst_path.to_string_lossy()
    );

    #[cfg(target_os = "linux")]
    println!(
        "cargo:rustc-link-search={}/build",
        &tinyinst_path.to_string_lossy()
    );
    println!(
        "cargo:rustc-link-search={}/build/third_party/obj/wkit/lib",
        &tinyinst_path.to_string_lossy()
    );
    println!("cargo:rustc-link-lib=static=tinyinst");

    #[cfg(target_arch = "x86_64")]
    println!("cargo:rustc-link-lib=static=xed");

    #[cfg(target_arch = "aarch64")]
    println!("cargo:rustc-link-lib=static=reil");

    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=dylib=dbghelp");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=src/tinyinst.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Tinyinst/litecov.cpp");
}

// #[cfg(not(target_os = "linux"))]
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

    // cmake file
    std::fs::copy("./src/CMakeLists.txt", tinyinst_path.join("CMakeLists.txt")).unwrap();
}
