// this file is first generated with ai, and then
// hand-edited & -written. i do not have this area of
// expertise, so if you find anything weird, please submit
// an issue.
//
// i know, i hate ai slop

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use heck::ToShoutySnakeCase;

const WEBKIT_VERSION: &str = "4a6a32c32c11ffb9f5a94c310b10f50130bfe6de";

#[derive(Debug)]
enum WebKitSource {
    /// build from a local source tree at vendor/WebKit
    Local { build_type: String },
    /// download a prebuilt tarball from oven-sh/WebKit releases
    Prebuilt { build_type: String },
}

impl WebKitSource {
    fn resolve() -> Self {
        // WEBKIT_SOURCE=system|local|prebuilt
        if let Ok(source) = env::var("WEBKIT_SOURCE") {
            let build_type = Self::build_type();
            return match source.to_lowercase().as_str() {
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

        // if let Some(name) = Self::system_headers_name() {
        //     return Self::System { name };
        // }

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
    println!("resolved WebKit source: {source:?}");

    let (webkit_path, webkit_lib_path, webkit_include_path) = match &source {
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
            (path, lib, inc)
        }
        WebKitSource::Prebuilt { build_type } => {
            let prefix = &WEBKIT_VERSION[..16];
            let path = env::var("WEBKIT_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| cache_path.join(format!("webkit-{}", prefix)));
            let lib = path.join("lib");
            let inc = path.join("include");
            download_prebuilt_webkit(&path, &cache_path, build_type);
            (path, lib, inc)
        }
    };

    // Link
    println!(
        "cargo:rustc-link-search=native={}",
        webkit_lib_path.display()
    );

    // windows
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
        webkit_lib_path.display()
    );

    // macos
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=c++");

        // fallback to system icu via -licucore
        println!("cargo:rustc-link-lib=icucore");

        println!("cargo:rustc-link-arg=-Wl,-all_load");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=atomic");
        println!("cargo:rustc-link-lib=icuuc");
        println!("cargo:rustc-link-lib=icui18n");
        println!("cargo:rustc-link-lib=icudata");
        println!("cargo:rustc-link-arg=-Wl,--whole-archive");
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

    generate_bindings(&webkit_include_path);
}

fn build_local_webkit(
    webkit_source_dir: &Path,
    webkit_path: &Path,
    webkit_lib_path: &Path,
    build_type: &str,
) {
    // windows: build icu from source
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

        // copy ICU libs to webkit_lib_path with expected names
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

    // jsc cmake args
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

    // windows-specific cmake args
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
fn download_prebuilt_webkit(webkit_path: &Path, cache_path: &Path, build_type: &str) {
    if webkit_path.exists() {
        println!("patched webkit already downloaded, skipping");
        return;
    }

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

    // download webkit
    {
        let trials: [Box<dyn FnOnce() -> bool>; 2] = [
            Box::new({
                let archive_path = archive_path.clone();
                let url = url.clone();
                move || {
                    Command::new("curl")
                        .args(["-fL", "-o", &archive_path.to_string_lossy(), &url])
                        .status()
                        .ok() // Option<...>
                        .map(|item| item.success()) // Option<bool>
                        .is_some_and(|inner| inner)
                }
            }),
            Box::new({
                let archive_path = archive_path.clone();
                let url = url.clone();
                move || {
                    Command::new("python3")
                        .args(["scripts/download.py", &url, &archive_path.to_string_lossy()])
                        .status()
                        .ok()
                        .map(|item| item.success()) // Option<bool>
                        .is_some_and(|inner| inner)
                }
            }),
        ];

        let downloaded = 'rt: {
            for trial in trials.into_iter() {
                if trial() {
                    break 'rt true;
                }

                println!("cargo:warning=failed to download webkit, retrying different method");
            }

            false
        };
        if !downloaded {
            panic!("Failed to download WebKit");
        }
    }
    println!("cargo:warning=successfully downloaded webkit");

    // extract webkit: either with `tar` or with python
    {
        let trials: [Box<dyn FnOnce() -> bool>; 2] = [
            Box::new({
                let archive_path = archive_path.clone();

                move || {
                    Command::new("tar")
                        .args([
                            "-xzf",
                            &archive_path.to_string_lossy(),
                            "-C",
                            &cache_path.to_string_lossy(),
                        ])
                        .status()
                        .ok()
                        .map(|item| item.success())
                        .is_some_and(|inner| inner)
                }
            }),
            Box::new({
                let archive_path = archive_path.clone();
                move || {
                    Command::new("python3")
                        .args([
                            "scripts/extract.py",
                            &archive_path.to_string_lossy(),
                            &cache_path.to_string_lossy(),
                        ])
                        .status()
                        .ok()
                        .map(|item| item.success()) // Option<bool>
                        .is_some_and(|inner| inner)
                }
            }),
        ];

        let extracted = 'rt: {
            for trial in trials.into_iter() {
                if trial() {
                    break 'rt true;
                }

                println!("cargo:warning=failed to download webkit, retrying different method");
            }

            false
        };
        if !extracted {
            panic!("failed to extract webkit");
        }
    }

    fs::remove_file(&archive_path).ok();
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

fn get_sdk_path() -> Option<String> {
    std::process::Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8(o.stdout).unwrap().trim().to_string())
}

fn sdk_flags() -> Vec<String> {
    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        if let Some(path) = get_sdk_path() {
            return vec![
                format!("-isysroot{}", path),
                "-fretain-comments-from-system-headers".to_string(),
            ];
        }
    }
    vec![]
}

fn jsc_include_flags(webkit_include_path: &PathBuf) -> Vec<String> {
    let javascript_h = webkit_include_path.join("JavaScriptCore/JavaScript.h");
    let mut options = vec![
        format!("-I{}", webkit_include_path.display()),
        format!("-DJSC_INCLUDE_PATH=\"{}\"", javascript_h.to_string_lossy()),
    ];

    if cfg!(feature = "remote-inspector") {
        let ri_h = webkit_include_path.join("JavaScriptCore/JSRemoteInspector.h");
        options.push(format!("-DRI_INCLUDE_PATH=\"{}\"", ri_h.to_string_lossy()))
    }

    if javascript_h.exists() {
        return options;
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
            ItemKind::Function | ItemKind::Module => Some(item_info.name.to_snake_case()),
            ItemKind::Var => Some(item_info.name.to_shouty_snake_case()),
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

fn generate_bindings(webkit_include_path: &PathBuf) {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let bindings = sdk_flags()
        .into_iter()
        .chain(jsc_include_flags(webkit_include_path))
        .fold(
            {
                let builder = bindgen::Builder::default()
                    .header("wrapper.h")
                    .allowlist_recursively(true)
                    .allowlist_function("JS.*")
                    .allowlist_item("JS.*")
                    .allowlist_type("JS.*")
                    .allowlist_var("kJS.*")
                    .blocklist_type("pid_t")
                    .no_copy("OpaqueJS.*")
                    .generate_comments(true)
                    .default_alias_style(bindgen::AliasVariation::TypeAlias)
                    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                    .parse_callbacks(BindgenCallback::new())
                    .clang_arg(format!("-I{}", webkit_include_path.display()));

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
