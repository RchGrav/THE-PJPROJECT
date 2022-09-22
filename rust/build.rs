extern crate bindgen;
use std::process::Command;
//use bindgen::EnumVariation;

pub fn main() {
    println!("cargo:rerun-if-changed=../pjproject/pjsip/include/pjsua.h");
    println!("cargo:rerun-if-changed=build.rs");
    
    println!("cargo:rustc-link-search=../pjproject/pjsip/lib");
    println!("cargo:rustc-link-search=../pjproject/pjlib/lib");
    println!("cargo:rustc-link-search=../pjproject/pjlib-util/lib");
    println!("cargo:rustc-link-search=../pjproject/pjmedia/lib");
    println!("cargo:rustc-link-search=../pjproject/pjnath/lib");
    println!("cargo:rustc-link-search=../pjproject/third_party/lib");

    println!("cargo:rustc-link-lib=pj");
    println!("cargo:rustc-link-lib=pjsip-simple");
    println!("cargo:rustc-link-lib=pjsip-ua");
    println!("cargo:rustc-link-lib=pjsip");
    println!("cargo:rustc-link-lib=pjsua");
    println!("cargo:rustc-link-lib=pjsua2");
    println!("cargo:rustc-link-lib=pjlib-util");
    println!("cargo:rustc-link-lib=pjmedia-audiodev");
    println!("cargo:rustc-link-lib=pjmedia-codec");
    println!("cargo:rustc-link-lib=pjmedia-videodev");
    println!("cargo:rustc-link-lib=pjmedia");
    println!("cargo:rustc-link-lib=pjnath");

    println!("cargo:rustc-link-lib=g7221codec");
    println!("cargo:rustc-link-lib=gsmcodec");
    println!("cargo:rustc-link-lib=ilbccodec");
    println!("cargo:rustc-link-lib=resample");
    println!("cargo:rustc-link-lib=speex");
    println!("cargo:rustc-link-lib=srtp");
    println!("cargo:rustc-link-lib=webrtc");
    println!("cargo:rustc-link-lib=yuv");

    // Command::new("sh")
    //         .arg("-c")
    //         .arg("make dep")
    //         .current_dir("../pjproject")
    //         .output()
    //         .expect("failed to execute make dep");

    // Command::new("sh")
    //         .arg("-c")
    //         .arg("make")
    //         .current_dir("../pjproject")
    //         .output()
    //         .expect("failed to execute make");
            
    let bindings = bindgen::Builder::default()
        .header("../pjproject/pjsip/include/pjsua.h")
        .clang_args(["-I", "../pjproject/pjsip/include"])
        .clang_args(["-I", "../pjproject/pjlib/include"])
        .clang_args(["-I", "../pjproject/pjlib-util/include"])
        .clang_args(["-I", "../pjproject/pjmedia/include"])
        .clang_args(["-I", "../pjproject/pjnath/include"])
        .generate_comments(false)
        // .layout_tests(false)
        .allowlist_type(r"pj.*")
        .allowlist_type(r"PJ.*")
        .allowlist_var(r"pj.*")
        .allowlist_var(r"PJ.*")
        .allowlist_function(r"pj.*")
        .allowlist_function(r"PJ.*")
        .generate()
        .expect("Couldn't generate bindings!");

        let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
}