// file is generated with artificial intelligence and not yet edited.
// i will edit this once i get how they work.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const WEBKIT_VERSION: &str = "4a6a32c32c11ffb9f5a94c310b10f50130bfe6de";

#[derive(Debug)]
enum WebKitSource {
    /// Use whatever is installed on the system (e.g. macOS framework, apt package)
    System { name: &'static str },
    /// Build from a local source tree at vendor/WebKit
    Local { build_type: String },
    /// Download a prebuilt tarball from oven-sh/WebKit releases
    Prebuilt { build_type: String },
}

impl WebKitSource {
    fn resolve() -> Self {
        // WEBKIT_SOURCE=system|local|prebuilt
        if let Ok(source) = env::var("WEBKIT_SOURCE") {
            let build_type = Self::build_type();
            return match source.to_lowercase().as_str() {
                "system" => Self::System {
                    name: Self::system_headers_name().expect("couldn't find system headers"),
                },
                "local" => Self::Local { build_type },
                "prebuilt" => Self::Prebuilt { build_type },
                other => panic!(
                    "Unknown WEBKIT_SOURCE value: '{}'. Expected system, local, or prebuilt.",
                    other
                ),
            };
        }

        if env::var("WEBKIT_LOCAL").is_ok() {
            return Self::Local {
                build_type: Self::build_type(),
            };
        }

        if let Some(name) = Self::system_headers_name() {
            return Self::System { name };
        }

        Self::Prebuilt {
            build_type: Self::build_type(),
        }
    }

    fn build_type() -> String {
        env::var("WEBKIT_BUILD_TYPE").unwrap_or_else(|_| {
            if cfg!(debug_assertions) {
                "Debug".into()
            } else {
                "Release".into()
            }
        })
    }

    fn system_headers_name() -> Option<&'static str> {
        // macOS framework
        #[cfg(target_os = "macos")]
        {
            let pathname = "/System/Library/Frameworks/JavaScriptCore.framework";
            if fs::metadata(pathname).is_ok() {
                return Some("framework=JavascriptCore");
            }
        }

        // linux: common package paths (libjavascriptcoregtk-4.x-dev)
        #[cfg(target_os = "linux")]
        for (name, pathname) in &[
            (
                "javascriptcoregtk-4.1",
                "/usr/include/webkitgtk-4.1/JavaScriptCore/JavaScript.h",
            ),
            (
                "javascriptcoregtk-4.0",
                "/usr/include/webkitgtk-4.0/JavaScriptCore/JavaScript.h",
            ),
        ] {
            let path = Path::new(pathname);
            if path.exists() {
                return Some(name);
            }
        }

        None
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let vendor_path = manifest_dir.join("vendor");
    let cache_path = out_dir.join("cache");
    fs::create_dir_all(&cache_path).unwrap();

    let webkit_build_type = env::var("WEBKIT_BUILD_TYPE").unwrap_or_else(|_| {
        if cfg!(debug_assertions) {
            "Debug".to_string()
        } else {
            "Release".to_string()
        }
    });

    let source = WebKitSource::resolve();
    println!("cargo:warning=resolved WebKit source: {source:?}");

    let (webkit_path, webkit_lib_path, webkit_include_path) = match &source {
        WebKitSource::System { name: _ } => (None, None, None),
        WebKitSource::Local { build_type } => {
            let path = env::var("WEBKIT_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    vendor_path
                        .join("WebKit")
                        .join("WebKitBuild")
                        .join(build_type)
                });
            let lib = path.join("lib");
            let inc = path.join("include");
            build_local_webkit(&vendor_path.join("WebKit"), &path, &lib, build_type);
            (Some(path), Some(lib), Some(inc))
        }
        WebKitSource::Prebuilt { build_type } => {
            let prefix = &WEBKIT_VERSION[..16];
            let path = env::var("WEBKIT_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| cache_path.join(format!("webkit-{}", prefix)));
            let lib = path.join("lib");
            let inc = path.join("include");
            download_prebuilt_webkit(&path, &lib, &cache_path, build_type);
            (Some(path), Some(lib), Some(inc))
        }
    };

    // Link
    if let Some(lib_path) = &webkit_lib_path {
        let webkit_path = webkit_path.unwrap();

        println!("cargo:rustc-link-search=native={}", lib_path.display());
        if cfg!(target_os = "windows") {
            println!("cargo:rustc-link-lib=static=JavaScriptCore");
            println!("cargo:rustc-link-lib=static=WTF");
            println!("cargo:rustc-link-lib=static=bmalloc");

            // ICU static libs (prebuilt naming convention: 's' prefix)
            let icu_suffix = if webkit_build_type == "Debug" {
                "d"
            } else {
                ""
            };
            println!("cargo:rustc-link-lib=static=sicudt{}", icu_suffix);
            println!("cargo:rustc-link-lib=static=sicuin{}", icu_suffix);
            println!("cargo:rustc-link-lib=static=sicuuc{}", icu_suffix);
        } else {
            println!("cargo:rustc-link-lib=static=JavaScriptCore");
            println!("cargo:rustc-link-lib=static=WTF");
            println!("cargo:rustc-link-lib=static=bmalloc");
        }

        println!("cargo:include={}", webkit_path.display());
        println!(
            "cargo:include={}",
            webkit_path.join("JavaScriptCore/Headers").display()
        );
        println!(
            "cargo:include={}",
            webkit_path
                .join("JavaScriptCore/Headers/JavaScriptCore")
                .display()
        );
        println!(
            "cargo:include={}",
            webkit_path.join("JavaScriptCore/PrivateHeaders").display()
        );
        println!(
            "cargo:include={}",
            webkit_path.join("bmalloc/Headers").display()
        );
        println!(
            "cargo:include={}",
            webkit_path.join("WTF/Headers").display()
        );
        println!(
            "cargo:include={}",
            webkit_path
                .join("JavaScriptCore/PrivateHeaders/JavaScriptCore")
                .display()
        );

        println!(
            "cargo:rustc-link-search=native={}",
            webkit_lib_path.unwrap().display()
        );
    } else {
        if let WebKitSource::System { name } = &source {
            println!("cargo:rustc-link-lib={}", &name);
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        }
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=atomic");
        println!("cargo:rustc-link-lib=icuuc");
        println!("cargo:rustc-link-lib=icui18n");
        println!("cargo:rustc-link-lib=icudata");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=c++");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=crypt32");
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=dbghelp");
    }

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=WEBKIT_LOCAL");
    println!("cargo:rerun-if-env-changed=WEBKIT_PATH");
    println!("cargo:rerun-if-env-changed=WEBKIT_BUILD_TYPE");
    println!("cargo:rerun-if-env-changed=WEBKIT_VERSION");

    generate_bindings(webkit_include_path.as_ref());
}

/// Mirrors the WEBKIT_LOCAL branch of SetupWebKit.cmake
fn build_local_webkit(
    webkit_source_dir: &Path,
    webkit_path: &Path,
    webkit_lib_path: &Path,
    build_type: &str,
) {
    // Windows: build ICU from source (mirrors "Build ICU from source" block)
    let icu_local_root = if cfg!(target_os = "windows") {
        let root = webkit_source_dir.join("WebKitBuild").join("icu");
        let icu_lib = root.join("lib").join("sicudt.lib");
        if !icu_lib.exists() {
            println!("cargo:warning=Building ICU from source...");
            let arch = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
                "aarch64" => "ARM64",
                _ => "x64",
            };
            let status = Command::new("powershell")
                .args([
                    "-ExecutionPolicy",
                    "Bypass",
                    "-File",
                    &webkit_source_dir.join("build-icu.ps1").to_string_lossy(),
                    "-Platform",
                    arch,
                    "-BuildType",
                    build_type,
                    "-OutputDir",
                    &root.to_string_lossy(),
                ])
                .status()
                .expect("Failed to launch PowerShell for ICU build");
            assert!(status.success(), "ICU build failed");
        }

        // Copy ICU libs to webkit_lib_path with expected names
        // Mirrors: file(COPY_FILE ...) block
        fs::create_dir_all(webkit_lib_path).unwrap();
        let suffix = if build_type == "Debug" { "d" } else { "" };
        for (src, dst) in &[
            ("sicudt.lib", format!("sicudt{}.lib", suffix)),
            ("icuin.lib", format!("sicuin{}.lib", suffix)),
            ("icuuc.lib", format!("sicuuc{}.lib", suffix)),
        ] {
            let src_path = root.join("lib").join(src);
            let dst_path = webkit_lib_path.join(dst);
            if src_path != dst_path {
                fs::copy(&src_path, &dst_path)
                    .unwrap_or_else(|_| panic!("Failed to copy ICU lib: {}", src));
            }
        }
        Some(root)
    } else {
        None
    };

    // Mirrors: set(JSC_CMAKE_ARGS ...) block
    let cmake_bin = which_cmake();
    let mut cmake_args = vec![
        "-S".to_string(),
        webkit_source_dir.to_string_lossy().into_owned(),
        "-B".to_string(),
        webkit_path.to_string_lossy().into_owned(),
        "-DPORT=JSCOnly".to_string(),
        "-DENABLE_STATIC_JSC=ON".to_string(),
        "-DUSE_THIN_ARCHIVES=OFF".to_string(),
        "-DENABLE_FTL_JIT=ON".to_string(),
        "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON".to_string(),
        "-DUSE_BUN_JSC_ADDITIONS=ON".to_string(),
        "-DUSE_BUN_EVENT_LOOP=ON".to_string(),
        "-DENABLE_BUN_SKIP_FAILING_ASSERTIONS=ON".to_string(),
        "-DALLOW_LINE_AND_COLUMN_NUMBER_IN_BUILTINS=ON".to_string(),
        format!("-DCMAKE_BUILD_TYPE={}", build_type),
        "-DENABLE_REMOTE_INSPECTOR=ON".to_string(),
        "-DENABLE_MEDIA_SOURCE=OFF".to_string(),
        "-DENABLE_MEDIA_STREAM=OFF".to_string(),
        "-DENABLE_WEB_RTC=OFF".to_string(),
    ];

    // Windows-specific cmake args (mirrors the if(WIN32) block)
    if cfg!(target_os = "windows") {
        let icu_root = icu_local_root.unwrap();
        cmake_args.extend([
            format!("-DICU_ROOT={}", icu_root.display()),
            format!("-DICU_LIBRARY={}", icu_root.join("lib").display()),
            format!("-DICU_INCLUDE_DIR={}", icu_root.join("include").display()),
            "-DCMAKE_LINKER=lld-link".to_string(),
        ]);
        let msvc_runtime = if build_type == "Debug" {
            "MultiThreadedDebug"
        } else {
            "MultiThreaded"
        };
        cmake_args.extend([
            format!("-DCMAKE_MSVC_RUNTIME_LIBRARY={}", msvc_runtime),
            "-DCMAKE_C_FLAGS=/DU_STATIC_IMPLEMENTATION".to_string(),
            "-DCMAKE_CXX_FLAGS=/DU_STATIC_IMPLEMENTATION /clang:-fno-c++-static-destructors"
                .to_string(),
        ]);
    }

    // Configure
    let status = Command::new(&cmake_bin)
        .args(&cmake_args)
        .status()
        .expect("Failed to run cmake configure");
    assert!(status.success(), "JSC cmake configure failed");

    // Build (mirrors: add_custom_target(jsc ...))
    let status = Command::new(&cmake_bin)
        .args([
            "--build",
            &webkit_path.to_string_lossy(),
            "--config",
            build_type,
            "--target",
            "jsc",
        ])
        .status()
        .expect("Failed to run cmake build");
    assert!(status.success(), "JSC cmake build failed");
}

/// Mirrors the prebuilt download branch of SetupWebKit.cmake
fn download_prebuilt_webkit(
    webkit_path: &Path,
    _webkit_lib_path: &Path,
    cache_path: &Path,
    build_type: &str,
) {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        panic!("Unsupported OS")
    };

    let arch = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "aarch64" => "arm64",
        "x86_64" => "amd64",
        other => panic!("Unsupported arch: {}", other),
    };

    // Mirrors WEBKIT_SUFFIX logic
    let mut suffix = String::new();
    if cfg!(target_env = "musl") {
        suffix.push_str("-musl");
    }
    if build_type == "Debug" {
        suffix.push_str("-debug");
    }
    // ENABLE_LTO / ENABLE_ASAN would be additional env vars to check here

    // Mirrors: WEBKIT_NAME, WEBKIT_TAG, WEBKIT_DOWNLOAD_URL
    let name = format!("bun-webkit-{}-{}{}", os, arch, suffix);
    let filename = format!("{}.tar.gz", name);
    let tag = if WEBKIT_VERSION.starts_with("autobuild-") {
        WEBKIT_VERSION.to_string()
    } else {
        format!("autobuild-{}", WEBKIT_VERSION)
    };
    let url = format!(
        "https://github.com/oven-sh/WebKit/releases/download/{}/{}",
        tag, filename
    );

    let archive_path = cache_path.join(&filename);
    println!("cargo:warning=Downloading WebKit from {}", url);

    // Download
    let status = Command::new("curl")
        .args(["-fL", "-o", &archive_path.to_string_lossy(), &url])
        .status()
        .expect("Failed to run curl; ensure curl is available");
    assert!(status.success(), "Failed to download WebKit");

    // Extract (mirrors: file(ARCHIVE_EXTRACT ...))
    let status = Command::new("tar")
        .args([
            "-xzf",
            &archive_path.to_string_lossy(),
            "-C",
            &cache_path.to_string_lossy(),
        ])
        .status()
        .expect("Failed to extract WebKit archive");
    assert!(status.success(), "Failed to extract WebKit");

    fs::remove_file(&archive_path).ok();
    if webkit_path.exists() {
        fs::remove_dir_all(webkit_path).unwrap();
    }
    fs::rename(cache_path.join("bun-webkit"), webkit_path).unwrap();

    // Mirrors: if(APPLE) file(REMOVE_RECURSE unicode)
    if cfg!(target_os = "macos") {
        let unicode_dir = webkit_path.join("include").join("unicode");
        if unicode_dir.exists() {
            fs::remove_dir_all(unicode_dir).ok();
        }
    }
}

fn which_cmake() -> String {
    env::var("CMAKE").unwrap_or_else(|_| "cmake".to_string())
}

fn sdk_flags() -> Vec<String> {
    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        let output = std::process::Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .ok()
            .filter(|o| o.status.success());

        if let Some(out) = output {
            let path = std::str::from_utf8(&out.stdout).unwrap().trim().to_string();
            return vec![
                format!("-isysroot{}", path),
                "-fretain-comments-from-system-headers".to_string(),
            ];
        }
    }
    vec![]
}

fn jsc_include_flags(webkit_include_path: &PathBuf) -> Vec<String> {
    let downloaded = webkit_include_path.join("JavaScriptCore/JavaScript.h");
    if downloaded.exists() {
        return vec![
            format!("-I{}", webkit_include_path.display()),
            format!("-DJSC_INCLUDE_PATH=\"{}\"", downloaded.to_string_lossy()),
        ];
    }

    panic!(
        "JavaScriptCore headers not found.\n  \
         - checked system: /System/Library/Frameworks/JavaScriptCore.framework\n  \
         - checked downloaded: {}\n  \
         ...but nothing was found.
         ",
        webkit_include_path.display()
    );
}

#[derive(Debug)]
pub struct BindgenCallback;

impl BindgenCallback {
    #[inline(always)]
    fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl bindgen::callbacks::ParseCallbacks for BindgenCallback {
    fn item_name(&self, item_info: bindgen::callbacks::ItemInfo) -> Option<String> {
        use bindgen::callbacks::ItemKind;
        use heck::{ToSnakeCase, ToUpperCamelCase};

        match item_info.kind {
            ItemKind::Function | ItemKind::Module | ItemKind::Var => {
                Some(item_info.name.to_snake_case())
            }
            ItemKind::Type => Some(item_info.name.to_upper_camel_case()),
            _ => None,
        }
    }

    fn process_comment(&self, comment: &str) -> Option<String> {
        match doxygen_bindgen::transform(comment) {
            Ok(res) => Some(res),
            Err(err) => {
                println!("cargo:warning=Problem processing doxygen comment: {comment}\n{err}");
                None
            }
        }
    }
}

fn generate_bindings(webkit_include_path: Option<&PathBuf>) {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let bindings = sdk_flags()
        .into_iter()
        .chain({
            if let Some(ref include_path) = webkit_include_path {
                jsc_include_flags(include_path)
            } else {
                vec![]
            }
        })
        .fold(
            {
                let mut builder = bindgen::Builder::default()
                    .header("wrapper.h")
                    .allowlist_recursively(true)
                    .allowlist_function("JS.*")
                    .allowlist_item("JS.*")
                    .allowlist_type("JS.*")
                    .allowlist_var("kJS.*")
                    .no_copy("OpaqueJS.*")
                    .generate_comments(true)
                    .default_alias_style(bindgen::AliasVariation::TypeAlias)
                    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                    .parse_callbacks(BindgenCallback::new());

                if let Some(include_path) = webkit_include_path {
                    builder = builder.clang_arg(format!("-I{}", include_path.display()));
                }

                builder
            },
            |b, flag| b.clang_arg(flag),
        )
        .generate()
        .expect("Failed to generate JSC bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings.rs");
}
