use std::{
    collections::HashMap,
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
};

#[macro_export]
macro_rules! warning {
    ($format:expr $(, $($args:tt)*)?) => {
        writeln!(std::io::stdout(), concat!("[WARNING] ", $format) $(, $($args)*)?).unwrap()
    };
}

fn main() {
    let cargo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let circt_dir = &cargo_root.join("circt");
    Command::new("git")
        .args(&[
            "submodule",
            "update",
            "--init",
            "--recursive",
            circt_dir.to_str().unwrap(),
        ])
        .current_dir(cargo_root)
        .status()
        .expect("git submodule update failed!");

    let num_jobs = num_cpus::get();
    let env = HashMap::from([
        ("MAKEFLAGS", format!("-j {num_jobs}")),
        ("CMAKE_BUILD_PARALLEL_LEVEL", format!("{num_jobs}")),
    ]);

    let mut child = Command::new("bash")
        .arg(circt_dir.join("utils/get-or-tools.sh").to_str().unwrap())
        .stdin(Stdio::piped())
        .current_dir(circt_dir)
        .envs(&env)
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all("yes".as_bytes())
        .unwrap();
    assert!(child.wait().unwrap().success());

    let build_type = "Release";
    let generator = "Ninja";

    warning!("Building LLVM... (this could take a long time!)");
    let llvm_path = cmake::Config::new(circt_dir.join("llvm/llvm"))
        .define("LLVM_ENABLE_PROJECTS", "mlir")
        .define("LLVM_TARGETS_TO_BUILD", "host")
        .define("CMAKE_BUILD_TYPE", build_type)
        .generator(generator)
        .target("install")
        .build();

    warning!("[DONE] llvm_path={:?}", &llvm_path);

    warning!("Building CIRCT.. (this could take a long time!)");
    let circt_path = cmake::Config::new(circt_dir)
        .define("LLVM_DIR", &llvm_path)
        .define("MLIR_DIR", &llvm_path.parent().unwrap().join("mlir"))
        .define("CMAKE_BUILD_TYPE", build_type)
        .generator(generator)
        .target("install")
        .build();
    warning!("[DONE] circt_path={:?}", circt_path);
}
