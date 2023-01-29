use cargo_emit::{rerun_if_changed, rustc_link_lib, rustc_link_search, warning};
use cmake;

use std::{
    collections::HashMap,
    env,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

static CIRCT_DEP_SCRIPTS: [&str; 1] = ["utils/get-or-tools.sh"];

fn main() {
    rerun_if_changed!("build.rs");
    rerun_if_changed!("Cargo.lock");

    let build_dependencies = env::var("BUILD_DEPENDENCIES")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
    let build_circt = env::var("BUILD_CIRCT")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
    let git_submodule_update = env::var("GIT_SUBMODULE_UPDATE")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
    let build_type = std::env::var("CIRCT_BUILD_TYPE").unwrap_or("Release".to_string());
    let generator = std::env::var("CMAKE_GENERATOR").unwrap_or("Ninja".to_string());

    let cargo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let circt_src_dir = &cargo_root.join("circt");
    let circt_install_dir = &cargo_root.join("circt_build");
    let llvm_src_dir = &circt_src_dir.join("llvm/llvm");

    let include_dir = &circt_install_dir.join("include");
    let lib_dir = &circt_install_dir.join("lib");
    let bindings_dir = &circt_install_dir;

    let num_jobs = num_cpus::get();

    if git_submodule_update {
        warning!("Checking out git submodule...");
        Command::new("git")
            .args(&[
                "submodule",
                "update",
                "--init",
                "--recursive",
                circt_src_dir.to_str().unwrap(),
            ])
            .current_dir(cargo_root)
            .status()
            .expect("git submodule update failed!");
    }
    if build_dependencies {
        let env = HashMap::from([
            ("MAKEFLAGS", format!("-j {num_jobs}")),
            ("CMAKE_BUILD_PARALLEL_LEVEL", format!("{num_jobs}")),
        ]);

        for sh_script in CIRCT_DEP_SCRIPTS {
            let mut child = Command::new("bash")
                .arg(circt_src_dir.join(sh_script).to_str().unwrap())
                .stdin(Stdio::piped())
                .current_dir(circt_src_dir)
                .envs(&env)
                .spawn()
                .expect(format!("Failed to run script: {sh_script}").as_str());
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all("yes".as_bytes())
                .expect(format!("Failed write `yes` to stdin of script: {sh_script}").as_str());
            assert!(child
                .wait()
                .expect(format!("Failed wait for script: {sh_script}").as_str())
                .success());
        }
        warning!("Done building dependencies!");
    }

    if build_circt {
        warning!("Building LLVM... (this could take a long time!)");
        if !&circt_install_dir.exists() {
            std::fs::create_dir_all(&circt_install_dir).unwrap();
        }
        cmake::Config::new(&llvm_src_dir)
            .define("LLVM_TARGETS_TO_BUILD", "host")
            .define("LLVM_ENABLE_PROJECTS", "mlir")
            .define("LLVM_EXTERNAL_PROJECTS", "circt")
            .define("LLVM_EXTERNAL_CIRCT_SOURCE_DIR", circt_src_dir)
            .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
            .define("CMAKE_BUILD_TYPE", build_type)
            .out_dir(&circt_install_dir)
            .generator(generator)
            .build();
    };

    rustc_link_search!(lib_dir.to_str().unwrap() => "native");

    let lib_names = [
        "LLVMBinaryFormat",
        "LLVMBitstreamReader",
        "LLVMCore",
        "LLVMDemangle",
        "LLVMRemarks",
        "LLVMSupport",
        "MLIRIR",
        "MLIRAnalysis",
        "MLIRCAPIFunc",
        "MLIRCAPIIR",
        "MLIRCAPIControlFlow",
        "MLIRCallInterfaces",
        "MLIRControlFlowDialect",
        "MLIRControlFlowInterfaces",
        "MLIRFuncDialect",
        "MLIRIR",
        "MLIRInferTypeOpInterface",
        "MLIRInferIntRangeInterface",
        "MLIRPDLToPDLInterp",
        "MLIRParser",
        "MLIRPass",
        "MLIRRewrite",
        "MLIRSideEffectInterfaces",
        "MLIRSupport",
        "MLIRTransformUtils",
        "MLIRTransforms",
        "CIRCTCAPIComb",
        "CIRCTCAPIHW",
        "CIRCTCAPISV",
        "CIRCTCAPISeq",
        "CIRCTComb",
        "CIRCTHW",
        "CIRCTSV",
        "CIRCTSeq",
    ];
    for lib in lib_names {
        rustc_link_lib!(lib => "static");
    }
    if let Some(os) = env::var_os("CARGO_CFG_TARGET_OS")
        .map(|s| s.into_string().ok())
        .flatten()
    {
        match os.as_str() {
            "macos" => {
                rustc_link_lib!("c++");

                if let Ok(prefix) = env::var("HOMEBREW_PREFIX") {
                    rustc_link_search!(format!("{prefix}/lib") => "native");
                }
            }
            "linux" => rustc_link_lib!("stdc++"),
            _ => {}
        }
    }
    rustc_link_lib!("ncurses" => "static");

    bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_block(true)
        .opaque_type("std::.*")
        .clang_args(&["-I", include_dir.to_str().unwrap()])
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Additional wrapper code
    rerun_if_changed!("wrapper.cpp");
    cc::Build::new()
        .cpp(true)
        .file("wrapper.cpp")
        .flag_if_supported("-std=c++17")
        .include(include_dir)
        .warnings(false)
        .extra_warnings(false)
        .compile("circt-sys-wrapper");
}
