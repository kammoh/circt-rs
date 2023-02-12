use cargo_emit::{rerun_if_changed, rustc_link_lib, rustc_link_search, warning};

use std::{
    collections::HashMap,
    env,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

static CIRCT_DEP_SCRIPTS: [&str; 2] = [
    // need CAPNP_VER=0.9.2 on macos
    "utils/get-capnp.sh",
    "utils/get-or-tools.sh",
];

fn main() {
    rerun_if_changed!("build.rs");
    // rerun_if_changed!("Cargo.lock"); // for some reason reruns build every time!

    let build_circt = env::var("BUILD_CIRCT")
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

    if build_circt {
        warning!("Checking out git submodule...");
        Command::new("git")
            .args([
                "submodule",
                "update",
                "--init",
                "--recursive",
                circt_src_dir.to_str().unwrap(),
            ])
            .current_dir(cargo_root)
            .status()
            .expect("git submodule update failed!");

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

        warning!("Building LLVM... (this could take a long time!)");
        if !&circt_install_dir.exists() {
            std::fs::create_dir_all(circt_install_dir).unwrap();
        }
        cmake::Config::new(llvm_src_dir)
            .define("LLVM_TARGETS_TO_BUILD", "host")
            .define("LLVM_ENABLE_PROJECTS", "mlir")
            .define("LLVM_EXTERNAL_PROJECTS", "circt")
            .define("LLVM_EXTERNAL_CIRCT_SOURCE_DIR", circt_src_dir)
            .define("MLIR_INSTALL_AGGREGATE_OBJECTS", "OFF")
            .define("LLVM_ENABLE_ASSERTIONS", "ON")
            .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
            .define("CIRCT_BINDINGS_PYTHON_ENABLED", "ON")
            .define("CIRCT_ENABLE_FRONTENDS", "PyCDE")
            .define("CMAKE_BUILD_TYPE", build_type)
            .out_dir(circt_install_dir)
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
        "LLVMIRPrinter",
        "LLVMCFIVerify",
        "LLVMTarget",
        "LLVMTargetParser",
        "LLVMPasses",
        "LLVMMCDisassembler",
        "MLIRSupport",
        "MLIRToLLVMIRTranslationRegistration",
        "MLIRLLVMCommonConversion",
        "MLIRCAPIRegisterEverything",
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
        "MLIRTargetLLVMIRExport",
        "MLIRPDLDialect",
        "MLIRPDLInterpDialect",
        "MLIRParser",
        "MLIRPass",
        "MLIRRewrite",
        "MLIRSideEffectInterfaces",
        "MLIRTransformUtils",
        "MLIRTransforms",
        "MLIRArithTransforms",
        "MLIRArithUtils",
        "MLIRArithToLLVM",
        "MLIRArithDialect",
        "MLIRArithAttrToLLVMConversion",
        "MLIRAffineUtils",
        "MLIRAffineTransformOps",
        "MLIRAffineDialect",
        // "MLIRTensorDialect",
        // "MLIRCAPITensor",
        // "MLIRSparseTensorDialect",
        // "MLIRSparseTensorRuntime",
        // "MLIRSparseTensorPipelines",
        // "MLIRSparseTensorTransforms",
        // "MLIRSparseTensorUtils",
        // "MLIRCAPISparseTensor",
        // "MLIRTensorTilingInterfaceImpl",
        // "MLIRTensorTransforms",
        // "MLIRTensorUtils",
        // "MLIRTensorToLinalg",
        // "MLIRTensorInferTypeOpInterfaceImpl",
        // "MLIRMemRefDialect",
        // "MLIRMemRefTransforms",
        // "MLIRMemRefToLLVM",
        // "MLIRMemRefTransformOps",
        // "MLIRBufferizationToMemRef",
        // "MLIRMemRefUtils",
        "CIRCTSupport",
        "CIRCTHW",
        "CIRCTCAPIHWArith",
        "CIRCTHWArith",
        "CIRCTHWArithToHW",
        "CIRCTCAPIComb",
        "CIRCTCAPIHW",
        "CIRCTComb",
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
        // "CIRCTSystemC",
        // "CIRCTHWToSystemC",
        // "CIRCTExportSystemC",
        // "CIRCTSystemCTransforms",
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

            if let Ok(prefix) = env::var("HOMEBREW_PREFIX") {
                rustc_link_search!(format!("{prefix}/lib") => "native");
            }
        }
        "linux" => rustc_link_lib!("stdc++"),
        _ => {}
    }

    if let Ok(library) = pkg_config::probe_library("ncurses") {
        for p in library.link_paths {
            warning!("Adding library path: {}", p.display());
            rustc_link_search!(p.display() => "static");
            rustc_link_search!(p.display());
        }
        for p in library.framework_paths {
            warning!("Adding framework path: {}", p.display());
            rustc_link_search!(p.display() => "static");
            rustc_link_search!(p.display());
        }
    } else {
        // panic!("pkg-cinfig failed");
        match os.as_str() {
            "macos" => {
                let p = "/opt/homebrew/opt/ncurses/lib";
                rustc_link_search!(p => "static");
                rustc_link_search!(p);
            }
            _ => {}
        }
    }
    rustc_link_lib!("ncurses" => "static");

    bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_block(true)
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
