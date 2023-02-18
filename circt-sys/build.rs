use cargo_emit::{
    rerun_if_changed, rerun_if_env_changed, rustc_link_lib, rustc_link_search, warning,
};

use miette::{IntoDiagnostic, Result};

use std::{
    collections::HashMap,
    env,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn main() -> Result<()> {
    let cargo_root = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().into_diagnostic()?;
    rerun_if_changed!(cargo_root.join("build.rs").display());

    let circt_src_dir = &env::var("CIRCT_SRC_DIR")
        .map(|e| PathBuf::from(&e))
        .unwrap_or(cargo_root.join("circt"))
        .canonicalize()
        .into_diagnostic()?;

    let circt_prefix = if let Ok(prefix) = env::var("CIRCT_INSTALL_PREFIX") {
        PathBuf::from(prefix).canonicalize().into_diagnostic()?
    } else {
        circt_src_dir.parent().unwrap().join("install")
    };
    rerun_if_env_changed!("CIRCT_INSTALL_PREFIX");

    if env::var("BUILD_CIRCT")
        .ok()
        .and_then(|s| {
            s.to_lowercase()
                .parse::<bool>()
                .ok()
                .or_else(|| s.parse::<u8>().ok().map(|i| i != 0))
        })
        .unwrap_or(false)
    {
        warning!("Building CIRCT from {}", circt_src_dir.display());
        rerun_if_env_changed!("CIRCT_SRC_DIR");
        build_circt(circt_src_dir, &circt_prefix)?;
    }
    rerun_if_env_changed!("BUILD_CIRCT");

    let lib_dir = &circt_prefix.join("lib");
    let include_dir = &circt_prefix.join("include");

    link_libs(lib_dir)?;

    let bindings_dir = cargo_root.join("bindings");
    std::fs::create_dir_all(&bindings_dir).into_diagnostic()?;

    rerun_if_changed!("wrapper.h");
    bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_block(true)
        .generate_inline_functions(true)
        .clang_args(&["-I", include_dir.to_str().unwrap()])
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&bindings_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Additional wrapper code
    rerun_if_changed!("wrapper.cpp");
    cc::Build::new()
        .cpp(true)
        .file("wrapper.cpp")
        .flag("-std=c++17")
        .include(include_dir)
        .warnings(false)
        .extra_warnings(false)
        .compile("circt-sys-wrapper");

    Ok(())
}

fn build_circt(circt_src_dir: &Path, circt_install_dir: &Path) -> Result<()> {
    let cmake_build_dir = circt_src_dir.parent().unwrap().join("circt_build");
    let llvm_src_dir = circt_src_dir.join("llvm/llvm");

    let build_type = std::env::var("CIRCT_BUILD_TYPE").unwrap_or("Release".to_string());
    rerun_if_env_changed!("CIRCT_BUILD_TYPE");
    let generator = std::env::var("CMAKE_GENERATOR").unwrap_or("Ninja".to_string());
    let num_jobs = num_cpus::get();
    warning!("Checking out git submodule...");
    Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .current_dir(circt_src_dir)
        .status()
        .expect("git submodule update failed!");

    let env = HashMap::from([
        (
            "MAKEFLAGS",
            std::env::var("MAKEFLAGS").unwrap_or("".to_string()) + &format!(" -j {num_jobs}"),
        ),
        (
            "CMAKE_BUILD_PARALLEL_LEVEL",
            std::env::var("CMAKE_BUILD_PARALLEL_LEVEL").unwrap_or(num_jobs.to_string()),
        ),
    ]);

    let build_dep_scripts: Vec<&str> = vec![
        // "utils/get-capnp.sh", // need CAPNP_VER=0.9.2 on macos
        // "utils/get-or-tools.sh",
    ];

    for sh_script in build_dep_scripts {
        let mut child = Command::new("bash")
            .arg(circt_src_dir.join(sh_script).to_str().unwrap())
            .stdin(Stdio::piped())
            .current_dir(circt_src_dir)
            .envs(&env)
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to run script: {sh_script}"));
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all("yes".as_bytes())
            .unwrap_or_else(|_| panic!("Failed write `yes` to stdin of script: {sh_script}"));
        assert!(child
            .wait()
            .unwrap_or_else(|_| panic!("Failed wait for script: {sh_script}"))
            .success());
    }
    warning!("Done building dependencies!");

    warning!("Building CIRCT... (this could take a long time!)");
    if !&circt_install_dir.exists() {
        std::fs::create_dir_all(&circt_install_dir).into_diagnostic()?;
    }

    cmake::Config::new(llvm_src_dir)
        .define("CMAKE_BUILD_TYPE", build_type)
        .define("CMAKE_C_COMPILER", "clang")
        .define("CMAKE_CXX_COMPILER", "clang++")
        .define("CMAKE_INSTALL_PREFIX", &circt_install_dir)
        .define("LLVM_TARGETS_TO_BUILD", "host")
        .define("LLVM_ENABLE_PROJECTS", "mlir")
        .define("LLVM_EXTERNAL_PROJECTS", "circt")
        .define("LLVM_EXTERNAL_CIRCT_SOURCE_DIR", circt_src_dir)
        .define("MLIR_INSTALL_AGGREGATE_OBJECTS", "OFF")
        .define("LLVM_ENABLE_ASSERTIONS", "OFF")
        .define("LLVM_ENABLE_BINDINGS", "OFF")
        .define("LLVM_ENABLE_OCAMLDOC", "OFF")
        .define("LLVM_INSTALL_UTILS", "ON")
        .define("LLVM_OPTIMIZED_TABLEGEN", "ON")
        .define("LLVM_STATIC_LINK_CXX_STDLIB", "ON")
        .define("LLVM_ENABLE_TERMINFO", "OFF")
        .define("VERILATOR_DISABLE", "ON")
        .cxxflag("-Wno-deprecated-declarations")
        .cflag("-Wno-deprecated-declarations")
        .out_dir(&cmake_build_dir)
        .generator(generator)
        .build();
    Ok(())
}

fn link_libs(lib_dir: &Path) -> Result<()> {
    rustc_link_search!(lib_dir.to_str().unwrap());

    let lib_names = [
        "LLVMCore",
        "LLVMTargetParser",
        "LLVMBinaryFormat",
        "LLVMDemangle",
        "LLVMSupport",
        "MLIRSupport",
        "MLIRLLVMCommonConversion",
        "MLIRIR",
        "MLIRDialectUtils",
        "MLIRAnalysis",
        "MLIRCAPIIR",
        "MLIRCallInterfaces",
        "MLIRCAPIControlFlow",
        "MLIRCAPIFunc",
        "MLIRControlFlowDialect",
        "MLIRControlFlowInterfaces",
        "MLIRLoopLikeInterface",
        "MLIRFuncDialect",
        "MLIRFuncTransforms",
        "MLIRIR",
        "MLIRInferTypeOpInterface",
        "MLIRInferIntRangeInterface",
        "MLIRInferIntRangeCommon",
        "MLIRViewLikeInterface",
        "MLIRShapedOpInterfaces",
        "MLIRPDLToPDLInterp",
        "MLIRPDLDialect",
        "MLIRPDLInterpDialect",
        "MLIRParser",
        "MLIRAsmParser",
        "MLIRBytecodeReader",
        "MLIRBytecodeWriter",
        "MLIRPass",
        "MLIRRewrite",
        "MLIRSideEffectInterfaces",
        "MLIRTransformUtils",
        "MLIRTransforms",
        "MLIRMemRefDialect",
        "MLIRMemRefTransforms",
        "MLIRMemRefTransformOps",
        "MLIRArithTransforms",
        "MLIRArithDialect",
        "MLIRArithUtils",
        "MLIRAffineDialect",
        "MLIRAffineUtils",
        "MLIRAffineTransformOps",
        "MLIRRuntimeVerifiableOpInterface",
        "CIRCTSupport",
        "CIRCTTransforms",
        "CIRCTHW",
        "CIRCTCAPIHWArith",
        "CIRCTHWArith",
        "CIRCTHWArithToHW",
        "CIRCTPipelineToHW",
        "CIRCTCAPIHW",
        "CIRCTHWTransforms",
        "CIRCTHWToLLVM",
        "CIRCTHandshakeToHW",
        "CIRCTCAPIComb",
        "CIRCTComb",
        "CIRCTCombToLLVM",
        "CIRCTSeq",
        "CIRCTCAPISeq",
        "CIRCTSeqTransforms",
        "CIRCTFSM",
        "CIRCTCAPIFSM",
        "CIRCTFSMTransforms",
        "CIRCTFSMToSV",
        "CIRCTSV",
        "CIRCTCAPISV",
        "CIRCTSVTransforms",
        "CIRCTExportVerilog",
        "CIRCTCAPIExportVerilog",
        "CIRCTAffineToPipeline",
        "CIRCTCAPIFIRRTL",
        "CIRCTFIRRTL",
        "CIRCTExportChiselInterface",
        "CIRCTFIRRTLToHW",
        "CIRCTFIRRTLTransforms",
    ];
    for lib in lib_names {
        rustc_link_lib!(lib => "static");
    }

    let os = env::var_os("CARGO_CFG_TARGET_OS")
        .and_then(|s| s.into_string().ok())
        .unwrap_or_default();

    match os.as_str() {
        "macos" => {
            rustc_link_lib!("c++");
            rustc_link_lib!("curses");
        }
        "linux" => {
            rustc_link_lib!("stdc++");
            rustc_link_lib!("ncurses");
        }
        _ => {}
    }
    Ok(())
}
