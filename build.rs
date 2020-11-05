fn main() {
    // download the repository if it isn't already.
    if !std::path::Path::new("glew/.git").exists() {
        std::process::Command::new("git")
            .args(&["submodule", "update", "--init", "glew"])
            .status()
            .unwrap();
    }
    // clean the repository.
    std::process::Command::new("git")
        .current_dir("glew")
        .args(&["clean", "-fd"])
        .status()
        .unwrap();

    std::process::Command::new("make")
        .current_dir("glew/auto")
        .status()
        .unwrap();

    // compile the library using cmake.
    let dst = cmake::Config::new("glew/build/cmake")
        .profile("Release")
        .build();

    // generate bindings.
    let bindings = bindgen::builder()
        .header(dst.join("include/GL/glew.h").display().to_string())
        .generate_comments(false)
        .whitelist_var("glew.*")
        .whitelist_function("glew.*")
        .whitelist_type("glew.*")
        .generate()
        .unwrap();

    let out: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().parse().unwrap();
    bindings.write_to_file(out.join("bindings.rs")).unwrap();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=GLEW");

    println!("cargo:rustc-link-lib=X11");
}
