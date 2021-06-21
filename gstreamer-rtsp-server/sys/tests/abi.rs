// Generated by gir (https://github.com/gtk-rs/gir @ 0d08607)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7d95377)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 85bd06b)
// DO NOT EDIT

use gstreamer_rtsp_server_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gstreamer-rtsp-server-1.0"];

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
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
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
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
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
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
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
    (
        "GstRTSPAddress",
        Layout {
            size: size_of::<GstRTSPAddress>(),
            alignment: align_of::<GstRTSPAddress>(),
        },
    ),
    (
        "GstRTSPAddressFlags",
        Layout {
            size: size_of::<GstRTSPAddressFlags>(),
            alignment: align_of::<GstRTSPAddressFlags>(),
        },
    ),
    (
        "GstRTSPAddressPool",
        Layout {
            size: size_of::<GstRTSPAddressPool>(),
            alignment: align_of::<GstRTSPAddressPool>(),
        },
    ),
    (
        "GstRTSPAddressPoolClass",
        Layout {
            size: size_of::<GstRTSPAddressPoolClass>(),
            alignment: align_of::<GstRTSPAddressPoolClass>(),
        },
    ),
    (
        "GstRTSPAddressPoolResult",
        Layout {
            size: size_of::<GstRTSPAddressPoolResult>(),
            alignment: align_of::<GstRTSPAddressPoolResult>(),
        },
    ),
    (
        "GstRTSPAuth",
        Layout {
            size: size_of::<GstRTSPAuth>(),
            alignment: align_of::<GstRTSPAuth>(),
        },
    ),
    (
        "GstRTSPAuthClass",
        Layout {
            size: size_of::<GstRTSPAuthClass>(),
            alignment: align_of::<GstRTSPAuthClass>(),
        },
    ),
    (
        "GstRTSPClient",
        Layout {
            size: size_of::<GstRTSPClient>(),
            alignment: align_of::<GstRTSPClient>(),
        },
    ),
    (
        "GstRTSPClientClass",
        Layout {
            size: size_of::<GstRTSPClientClass>(),
            alignment: align_of::<GstRTSPClientClass>(),
        },
    ),
    (
        "GstRTSPContext",
        Layout {
            size: size_of::<GstRTSPContext>(),
            alignment: align_of::<GstRTSPContext>(),
        },
    ),
    (
        "GstRTSPFilterResult",
        Layout {
            size: size_of::<GstRTSPFilterResult>(),
            alignment: align_of::<GstRTSPFilterResult>(),
        },
    ),
    (
        "GstRTSPMedia",
        Layout {
            size: size_of::<GstRTSPMedia>(),
            alignment: align_of::<GstRTSPMedia>(),
        },
    ),
    (
        "GstRTSPMediaClass",
        Layout {
            size: size_of::<GstRTSPMediaClass>(),
            alignment: align_of::<GstRTSPMediaClass>(),
        },
    ),
    (
        "GstRTSPMediaFactory",
        Layout {
            size: size_of::<GstRTSPMediaFactory>(),
            alignment: align_of::<GstRTSPMediaFactory>(),
        },
    ),
    (
        "GstRTSPMediaFactoryClass",
        Layout {
            size: size_of::<GstRTSPMediaFactoryClass>(),
            alignment: align_of::<GstRTSPMediaFactoryClass>(),
        },
    ),
    (
        "GstRTSPMediaFactoryURI",
        Layout {
            size: size_of::<GstRTSPMediaFactoryURI>(),
            alignment: align_of::<GstRTSPMediaFactoryURI>(),
        },
    ),
    (
        "GstRTSPMediaFactoryURIClass",
        Layout {
            size: size_of::<GstRTSPMediaFactoryURIClass>(),
            alignment: align_of::<GstRTSPMediaFactoryURIClass>(),
        },
    ),
    (
        "GstRTSPMediaStatus",
        Layout {
            size: size_of::<GstRTSPMediaStatus>(),
            alignment: align_of::<GstRTSPMediaStatus>(),
        },
    ),
    (
        "GstRTSPMountPoints",
        Layout {
            size: size_of::<GstRTSPMountPoints>(),
            alignment: align_of::<GstRTSPMountPoints>(),
        },
    ),
    (
        "GstRTSPMountPointsClass",
        Layout {
            size: size_of::<GstRTSPMountPointsClass>(),
            alignment: align_of::<GstRTSPMountPointsClass>(),
        },
    ),
    (
        "GstRTSPOnvifClient",
        Layout {
            size: size_of::<GstRTSPOnvifClient>(),
            alignment: align_of::<GstRTSPOnvifClient>(),
        },
    ),
    (
        "GstRTSPOnvifClientClass",
        Layout {
            size: size_of::<GstRTSPOnvifClientClass>(),
            alignment: align_of::<GstRTSPOnvifClientClass>(),
        },
    ),
    (
        "GstRTSPOnvifMedia",
        Layout {
            size: size_of::<GstRTSPOnvifMedia>(),
            alignment: align_of::<GstRTSPOnvifMedia>(),
        },
    ),
    (
        "GstRTSPOnvifMediaClass",
        Layout {
            size: size_of::<GstRTSPOnvifMediaClass>(),
            alignment: align_of::<GstRTSPOnvifMediaClass>(),
        },
    ),
    (
        "GstRTSPOnvifMediaFactory",
        Layout {
            size: size_of::<GstRTSPOnvifMediaFactory>(),
            alignment: align_of::<GstRTSPOnvifMediaFactory>(),
        },
    ),
    (
        "GstRTSPOnvifMediaFactoryClass",
        Layout {
            size: size_of::<GstRTSPOnvifMediaFactoryClass>(),
            alignment: align_of::<GstRTSPOnvifMediaFactoryClass>(),
        },
    ),
    (
        "GstRTSPOnvifServer",
        Layout {
            size: size_of::<GstRTSPOnvifServer>(),
            alignment: align_of::<GstRTSPOnvifServer>(),
        },
    ),
    (
        "GstRTSPOnvifServerClass",
        Layout {
            size: size_of::<GstRTSPOnvifServerClass>(),
            alignment: align_of::<GstRTSPOnvifServerClass>(),
        },
    ),
    (
        "GstRTSPPermissions",
        Layout {
            size: size_of::<GstRTSPPermissions>(),
            alignment: align_of::<GstRTSPPermissions>(),
        },
    ),
    (
        "GstRTSPPublishClockMode",
        Layout {
            size: size_of::<GstRTSPPublishClockMode>(),
            alignment: align_of::<GstRTSPPublishClockMode>(),
        },
    ),
    (
        "GstRTSPServer",
        Layout {
            size: size_of::<GstRTSPServer>(),
            alignment: align_of::<GstRTSPServer>(),
        },
    ),
    (
        "GstRTSPServerClass",
        Layout {
            size: size_of::<GstRTSPServerClass>(),
            alignment: align_of::<GstRTSPServerClass>(),
        },
    ),
    (
        "GstRTSPSession",
        Layout {
            size: size_of::<GstRTSPSession>(),
            alignment: align_of::<GstRTSPSession>(),
        },
    ),
    (
        "GstRTSPSessionClass",
        Layout {
            size: size_of::<GstRTSPSessionClass>(),
            alignment: align_of::<GstRTSPSessionClass>(),
        },
    ),
    (
        "GstRTSPSessionMedia",
        Layout {
            size: size_of::<GstRTSPSessionMedia>(),
            alignment: align_of::<GstRTSPSessionMedia>(),
        },
    ),
    (
        "GstRTSPSessionMediaClass",
        Layout {
            size: size_of::<GstRTSPSessionMediaClass>(),
            alignment: align_of::<GstRTSPSessionMediaClass>(),
        },
    ),
    (
        "GstRTSPSessionPool",
        Layout {
            size: size_of::<GstRTSPSessionPool>(),
            alignment: align_of::<GstRTSPSessionPool>(),
        },
    ),
    (
        "GstRTSPSessionPoolClass",
        Layout {
            size: size_of::<GstRTSPSessionPoolClass>(),
            alignment: align_of::<GstRTSPSessionPoolClass>(),
        },
    ),
    (
        "GstRTSPStream",
        Layout {
            size: size_of::<GstRTSPStream>(),
            alignment: align_of::<GstRTSPStream>(),
        },
    ),
    (
        "GstRTSPStreamClass",
        Layout {
            size: size_of::<GstRTSPStreamClass>(),
            alignment: align_of::<GstRTSPStreamClass>(),
        },
    ),
    (
        "GstRTSPStreamTransport",
        Layout {
            size: size_of::<GstRTSPStreamTransport>(),
            alignment: align_of::<GstRTSPStreamTransport>(),
        },
    ),
    (
        "GstRTSPStreamTransportClass",
        Layout {
            size: size_of::<GstRTSPStreamTransportClass>(),
            alignment: align_of::<GstRTSPStreamTransportClass>(),
        },
    ),
    (
        "GstRTSPSuspendMode",
        Layout {
            size: size_of::<GstRTSPSuspendMode>(),
            alignment: align_of::<GstRTSPSuspendMode>(),
        },
    ),
    (
        "GstRTSPThread",
        Layout {
            size: size_of::<GstRTSPThread>(),
            alignment: align_of::<GstRTSPThread>(),
        },
    ),
    (
        "GstRTSPThreadPool",
        Layout {
            size: size_of::<GstRTSPThreadPool>(),
            alignment: align_of::<GstRTSPThreadPool>(),
        },
    ),
    (
        "GstRTSPThreadPoolClass",
        Layout {
            size: size_of::<GstRTSPThreadPoolClass>(),
            alignment: align_of::<GstRTSPThreadPoolClass>(),
        },
    ),
    (
        "GstRTSPThreadType",
        Layout {
            size: size_of::<GstRTSPThreadType>(),
            alignment: align_of::<GstRTSPThreadType>(),
        },
    ),
    (
        "GstRTSPToken",
        Layout {
            size: size_of::<GstRTSPToken>(),
            alignment: align_of::<GstRTSPToken>(),
        },
    ),
    (
        "GstRTSPTransportMode",
        Layout {
            size: size_of::<GstRTSPTransportMode>(),
            alignment: align_of::<GstRTSPTransportMode>(),
        },
    ),
    (
        "GstSDPInfo",
        Layout {
            size: size_of::<GstSDPInfo>(),
            alignment: align_of::<GstSDPInfo>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(guint) GST_RTSP_ADDRESS_FLAG_EVEN_PORT", "4"),
    ("(guint) GST_RTSP_ADDRESS_FLAG_IPV4", "1"),
    ("(guint) GST_RTSP_ADDRESS_FLAG_IPV6", "2"),
    ("(guint) GST_RTSP_ADDRESS_FLAG_MULTICAST", "8"),
    ("(guint) GST_RTSP_ADDRESS_FLAG_NONE", "0"),
    ("(guint) GST_RTSP_ADDRESS_FLAG_UNICAST", "16"),
    ("GST_RTSP_ADDRESS_POOL_ANY_IPV4", "0.0.0.0"),
    ("GST_RTSP_ADDRESS_POOL_ANY_IPV6", "::"),
    ("(gint) GST_RTSP_ADDRESS_POOL_EINVAL", "-1"),
    ("(gint) GST_RTSP_ADDRESS_POOL_ELAST", "-4"),
    ("(gint) GST_RTSP_ADDRESS_POOL_ERANGE", "-3"),
    ("(gint) GST_RTSP_ADDRESS_POOL_ERESERVED", "-2"),
    ("(gint) GST_RTSP_ADDRESS_POOL_OK", "0"),
    ("GST_RTSP_AUTH_CHECK_CONNECT", "auth.check.connect"),
    (
        "GST_RTSP_AUTH_CHECK_MEDIA_FACTORY_ACCESS",
        "auth.check.media.factory.access",
    ),
    (
        "GST_RTSP_AUTH_CHECK_MEDIA_FACTORY_CONSTRUCT",
        "auth.check.media.factory.construct",
    ),
    (
        "GST_RTSP_AUTH_CHECK_TRANSPORT_CLIENT_SETTINGS",
        "auth.check.transport.client-settings",
    ),
    ("GST_RTSP_AUTH_CHECK_URL", "auth.check.url"),
    ("(gint) GST_RTSP_FILTER_KEEP", "1"),
    ("(gint) GST_RTSP_FILTER_REF", "2"),
    ("(gint) GST_RTSP_FILTER_REMOVE", "0"),
    ("(gint) GST_RTSP_MEDIA_STATUS_ERROR", "5"),
    ("(gint) GST_RTSP_MEDIA_STATUS_PREPARED", "3"),
    ("(gint) GST_RTSP_MEDIA_STATUS_PREPARING", "2"),
    ("(gint) GST_RTSP_MEDIA_STATUS_SUSPENDED", "4"),
    ("(gint) GST_RTSP_MEDIA_STATUS_UNPREPARED", "0"),
    ("(gint) GST_RTSP_MEDIA_STATUS_UNPREPARING", "1"),
    (
        "GST_RTSP_ONVIF_BACKCHANNEL_REQUIREMENT",
        "www.onvif.org/ver20/backchannel",
    ),
    ("GST_RTSP_ONVIF_REPLAY_REQUIREMENT", "onvif-replay"),
    ("GST_RTSP_PERM_MEDIA_FACTORY_ACCESS", "media.factory.access"),
    (
        "GST_RTSP_PERM_MEDIA_FACTORY_CONSTRUCT",
        "media.factory.construct",
    ),
    ("(gint) GST_RTSP_PUBLISH_CLOCK_MODE_CLOCK", "1"),
    ("(gint) GST_RTSP_PUBLISH_CLOCK_MODE_CLOCK_AND_OFFSET", "2"),
    ("(gint) GST_RTSP_PUBLISH_CLOCK_MODE_NONE", "0"),
    ("(gint) GST_RTSP_SUSPEND_MODE_NONE", "0"),
    ("(gint) GST_RTSP_SUSPEND_MODE_PAUSE", "1"),
    ("(gint) GST_RTSP_SUSPEND_MODE_RESET", "2"),
    ("(gint) GST_RTSP_THREAD_TYPE_CLIENT", "0"),
    ("(gint) GST_RTSP_THREAD_TYPE_MEDIA", "1"),
    ("GST_RTSP_TOKEN_MEDIA_FACTORY_ROLE", "media.factory.role"),
    (
        "GST_RTSP_TOKEN_TRANSPORT_CLIENT_SETTINGS",
        "transport.client-settings",
    ),
    ("(guint) GST_RTSP_TRANSPORT_MODE_PLAY", "1"),
    ("(guint) GST_RTSP_TRANSPORT_MODE_RECORD", "2"),
];
