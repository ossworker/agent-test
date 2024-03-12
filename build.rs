use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=include/wrapper.h");
    println!("cargo:rerun-if-changed=JAVA_HOME");
    if let Ok(java_home) = env::var("JAVA_HOME") {
        println!("java_home={}", java_home);
        let bindings = bindgen::Builder::default()
            .derive_debug(true)
            .clang_arg(clang_include(&java_include()))
            .clang_arg(clang_include(&java_include_platform()))
            .clang_arg("-v")
            .header(jvmti_wrapper())
            .with_codegen_config(bindgen::CodegenConfig::all())
            .allowlist_recursively(true)
            .generate_block(true)
            .prepend_enum_name(true)
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(jvmti_bindings())
            .expect("Unable to write bindings");
    } else {
        println!("cargo:warning=JAVA_HOME is not set, skipping bindings generation. The build may fail.")
    }
}

fn clang_include(path: &PathBuf) -> String {
    format!("-I{}", path.to_str().unwrap())
}

fn java_include() -> PathBuf {
    PathBuf::from(env::var("JAVA_HOME").unwrap()).join("include")
}

fn java_include_platform() -> PathBuf {
    if cfg!(target_os = "macos") {
        java_include().join("darwin")
    } else if cfg!(target_os = "windows") {
        java_include().join("win32")
    } else if cfg!(target_os = "linux") {
        java_include().join("linux")
    } else if cfg!(target_os = "freebsd") {
        java_include().join("freebsd")
    }else if cfg!(target_os = "openbsd") {
        java_include().join("openbsd")
    }else {
        panic!("Unsupported OS");
    }
}

fn jvmti_bindings() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap()).join("jvmti_bindings.rs")
}

fn jvmti_wrapper() -> String {
    String::from("include/wrapper.h")
}

