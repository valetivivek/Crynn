use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=include/gecko_wrapper.h");
    println!("cargo:rerun-if-changed=src/gecko_wrapper.cpp");

    // Find GeckoView using pkg-config
    let has_geckoview = pkg_config::Config::new()
        .atleast_version("91.0")
        .probe("geckoview")
        .is_ok();
    
    if has_geckoview {
        if let Ok(lib) = pkg_config::Config::new()
            .atleast_version("91.0")
            .probe("geckoview")
        {
            // Add include paths from pkg-config
            for include_path in &lib.include_paths {
                println!("cargo:rustc-link-search=native={}", include_path.display());
            }
            
            // Add library paths
            for lib_path in &lib.link_paths {
                println!("cargo:rustc-link-search=native={}", lib_path.display());
            }
            
            // Link against GeckoView libraries
            println!("cargo:rustc-link-lib=geckoview");
            println!("cargo:rustc-link-lib=geckoview_core");
            
            // Add compile-time defines
            println!("cargo:rustc-cfg=have_geckoview");
        }
    } else {
        // Fallback: try to find Gecko manually
        let gecko_paths = vec![
            "/usr/lib/firefox",
            "/usr/local/lib/firefox", 
            "/opt/firefox",
            "/Applications/Firefox.app/Contents/MacOS",
            "C:\\Program Files\\Mozilla Firefox",
            "C:\\Program Files (x86)\\Mozilla Firefox",
        ];
        
        let mut found_gecko = false;
        for path in gecko_paths {
            let gecko_path = PathBuf::from(path);
            if gecko_path.exists() {
                println!("cargo:rustc-link-search=native={}", gecko_path.display());
                println!("cargo:rustc-link-lib=xul");
                println!("cargo:rustc-cfg=have_gecko_fallback");
                found_gecko = true;
                break;
            }
        }
        
        if !found_gecko {
            println!("cargo:rustc-cfg=no_gecko");
            println!("cargo:warning=GeckoView not found - building in fallback mode");
        }
    }

    // Get output path first
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // In fallback mode (no GeckoView), skip bindgen and use manual bindings
    if !has_geckoview {
        println!("cargo:warning=GeckoView not found, using fallback bindings");
        let fallback_bindings = r#"
// Fallback bindings for Gecko FFI when GeckoView is not available
use std::ffi::c_void;

extern "C" {
    pub fn gecko_init() -> i32;
    pub fn gecko_shutdown();
    pub fn gecko_create_window(width: i32, height: i32, title: *const i8) -> *mut c_void;
    pub fn gecko_destroy_window(window: *mut c_void);
    pub fn gecko_resize_window(window: *mut c_void, width: i32, height: i32);
    pub fn gecko_navigate_to(window: *mut c_void, url: *const i8) -> i32;
    pub fn gecko_go_back(window: *mut c_void) -> i32;
    pub fn gecko_go_forward(window: *mut c_void) -> i32;
    pub fn gecko_reload(window: *mut c_void) -> i32;
    pub fn gecko_stop(window: *mut c_void) -> i32;
    pub fn gecko_create_tab(window: *mut c_void) -> *mut c_void;
    pub fn gecko_close_tab(window: *mut c_void, tab: *mut c_void);
    pub fn gecko_switch_to_tab(window: *mut c_void, tab: *mut c_void) -> i32;
    pub fn gecko_get_memory_usage(window: *mut c_void) -> usize;
    pub fn gecko_garbage_collect(window: *mut c_void);
}
"#;
        std::fs::write(
            &out_path.join("bindings.rs"),
            fallback_bindings
        ).expect("Failed to write fallback bindings");
        // Continue to compile C++ wrapper (which will work in fallback mode)
    } else {
        // Generate bindings using bindgen when GeckoView is available
        let mut builder = bindgen::Builder::default()
            .header("include/gecko_wrapper.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .clang_arg("-std=c++17")
            .allowlist_function("gecko_.*")
            .allowlist_type("gecko_.*");

        // Add GeckoView include paths
        if let Ok(lib) = pkg_config::Config::new()
            .atleast_version("91.0")
            .probe("geckoview")
        {
            for include_path in &lib.include_paths {
                builder = builder.clang_arg(format!("-I{}", include_path.display()));
            }
        }

        let bindings = builder
            .generate()
            .expect("Unable to generate bindings");
        
        // Write the bindings to the $OUT_DIR/bindings.rs file
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }

    // Compile the C++ wrapper
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("src/gecko_wrapper.cpp")
        .include("include")
        .flag("-std=c++17")
        .flag("-fPIC");

    // Add GeckoView include paths if found
    if has_geckoview {
        if let Ok(lib) = pkg_config::Config::new()
            .atleast_version("91.0")
            .probe("geckoview")
        {
            for include_path in &lib.include_paths {
                build.include(include_path);
            }
            build.define("HAVE_GECKOVIEW", "1");
        }
    } else {
        // Add fallback defines
        build.define("NO_GECKO", "1");
    }

    build.compile("gecko_wrapper");
}
