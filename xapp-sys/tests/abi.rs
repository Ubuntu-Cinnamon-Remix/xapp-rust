// Generated by gir (https://github.com/gtk-rs/gir @ b5068ede6c51)
// from ../../gir-files (@ 248ad6976459)
// DO NOT EDIT

use xapp_sys::*;
use std::mem::{align_of, size_of};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["xapp"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG")
        .unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in
        RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("XAppFavoriteInfo", Layout {size: size_of::<XAppFavoriteInfo>(), alignment: align_of::<XAppFavoriteInfo>()}),
    ("XAppFavoritesClass", Layout {size: size_of::<XAppFavoritesClass>(), alignment: align_of::<XAppFavoritesClass>()}),
    ("XAppGtkWindow", Layout {size: size_of::<XAppGtkWindow>(), alignment: align_of::<XAppGtkWindow>()}),
    ("XAppGtkWindowClass", Layout {size: size_of::<XAppGtkWindowClass>(), alignment: align_of::<XAppGtkWindowClass>()}),
    ("XAppIconChooserButtonClass", Layout {size: size_of::<XAppIconChooserButtonClass>(), alignment: align_of::<XAppIconChooserButtonClass>()}),
    ("XAppIconChooserDialogClass", Layout {size: size_of::<XAppIconChooserDialogClass>(), alignment: align_of::<XAppIconChooserDialogClass>()}),
    ("XAppIconSize", Layout {size: size_of::<XAppIconSize>(), alignment: align_of::<XAppIconSize>()}),
    ("XAppKbdLayoutController", Layout {size: size_of::<XAppKbdLayoutController>(), alignment: align_of::<XAppKbdLayoutController>()}),
    ("XAppKbdLayoutControllerClass", Layout {size: size_of::<XAppKbdLayoutControllerClass>(), alignment: align_of::<XAppKbdLayoutControllerClass>()}),
    ("XAppMonitorBlankerClass", Layout {size: size_of::<XAppMonitorBlankerClass>(), alignment: align_of::<XAppMonitorBlankerClass>()}),
    ("XAppObjectIface", Layout {size: size_of::<XAppObjectIface>(), alignment: align_of::<XAppObjectIface>()}),
    ("XAppObjectManagerClient", Layout {size: size_of::<XAppObjectManagerClient>(), alignment: align_of::<XAppObjectManagerClient>()}),
    ("XAppObjectManagerClientClass", Layout {size: size_of::<XAppObjectManagerClientClass>(), alignment: align_of::<XAppObjectManagerClientClass>()}),
    ("XAppObjectProxy", Layout {size: size_of::<XAppObjectProxy>(), alignment: align_of::<XAppObjectProxy>()}),
    ("XAppObjectProxyClass", Layout {size: size_of::<XAppObjectProxyClass>(), alignment: align_of::<XAppObjectProxyClass>()}),
    ("XAppObjectSkeleton", Layout {size: size_of::<XAppObjectSkeleton>(), alignment: align_of::<XAppObjectSkeleton>()}),
    ("XAppObjectSkeletonClass", Layout {size: size_of::<XAppObjectSkeletonClass>(), alignment: align_of::<XAppObjectSkeletonClass>()}),
    ("XAppPreferencesWindow", Layout {size: size_of::<XAppPreferencesWindow>(), alignment: align_of::<XAppPreferencesWindow>()}),
    ("XAppPreferencesWindowClass", Layout {size: size_of::<XAppPreferencesWindowClass>(), alignment: align_of::<XAppPreferencesWindowClass>()}),
    ("XAppScrollDirection", Layout {size: size_of::<XAppScrollDirection>(), alignment: align_of::<XAppScrollDirection>()}),
    ("XAppStackSidebarClass", Layout {size: size_of::<XAppStackSidebarClass>(), alignment: align_of::<XAppStackSidebarClass>()}),
    ("XAppStatusIconClass", Layout {size: size_of::<XAppStatusIconClass>(), alignment: align_of::<XAppStatusIconClass>()}),
    ("XAppStatusIconInterfaceIface", Layout {size: size_of::<XAppStatusIconInterfaceIface>(), alignment: align_of::<XAppStatusIconInterfaceIface>()}),
    ("XAppStatusIconInterfaceProxy", Layout {size: size_of::<XAppStatusIconInterfaceProxy>(), alignment: align_of::<XAppStatusIconInterfaceProxy>()}),
    ("XAppStatusIconInterfaceProxyClass", Layout {size: size_of::<XAppStatusIconInterfaceProxyClass>(), alignment: align_of::<XAppStatusIconInterfaceProxyClass>()}),
    ("XAppStatusIconInterfaceSkeleton", Layout {size: size_of::<XAppStatusIconInterfaceSkeleton>(), alignment: align_of::<XAppStatusIconInterfaceSkeleton>()}),
    ("XAppStatusIconInterfaceSkeletonClass", Layout {size: size_of::<XAppStatusIconInterfaceSkeletonClass>(), alignment: align_of::<XAppStatusIconInterfaceSkeletonClass>()}),
    ("XAppStatusIconMonitorClass", Layout {size: size_of::<XAppStatusIconMonitorClass>(), alignment: align_of::<XAppStatusIconMonitorClass>()}),
    ("XAppStatusIconState", Layout {size: size_of::<XAppStatusIconState>(), alignment: align_of::<XAppStatusIconState>()}),
    ("XAppStyleManagerClass", Layout {size: size_of::<XAppStyleManagerClass>(), alignment: align_of::<XAppStyleManagerClass>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) XAPP_ICON_SIZE_16", "16"),
    ("(gint) XAPP_ICON_SIZE_22", "22"),
    ("(gint) XAPP_ICON_SIZE_24", "24"),
    ("(gint) XAPP_ICON_SIZE_32", "32"),
    ("(gint) XAPP_ICON_SIZE_48", "48"),
    ("(gint) XAPP_ICON_SIZE_96", "96"),
    ("(gint) XAPP_SCROLL_DOWN", "1"),
    ("(gint) XAPP_SCROLL_LEFT", "2"),
    ("(gint) XAPP_SCROLL_RIGHT", "3"),
    ("(gint) XAPP_SCROLL_UP", "0"),
    ("(gint) XAPP_STATUS_ICON_STATE_FALLBACK", "1"),
    ("(gint) XAPP_STATUS_ICON_STATE_NATIVE", "0"),
    ("(gint) XAPP_STATUS_ICON_STATE_NO_SUPPORT", "2"),
];

