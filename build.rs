extern crate cc;

use std::env;
use std::path;

struct Version {
    major: String,
    minor: String,
    git: String,
}

fn stringify(err: env::VarError) -> String {
    format!("{}", err)
}

fn extract_git_version(pre: &str) -> Result<String, String> {
    // If a pre-release version is set, it is expected to have the format
    // pre.v<maj>.<min>.<n>.g<hash>, where <maj> and <min> are the last major and minor version,
    // <n> is the number of commits since this version and <hash> is the hash of the last commit.
    let parts: Vec<&str> = pre.split('.').collect();
    if parts.len() != 5 {
        return Err(format!("'{}' is not a valid pre-release version", pre));
    }
    Ok(format!("{}.{}-{}-{}", parts[1], parts[2], parts[3], parts[4]))
}

fn get_version() -> Result<Version, String> {
    let major = env::var("CARGO_PKG_VERSION_MAJOR").map_err(stringify)?;
    let minor = env::var("CARGO_PKG_VERSION_MINOR").map_err(stringify)?;
    let pre = env::var("CARGO_PKG_VERSION_PRE").map_err(stringify)?;

    let git = match pre.is_empty() {
        true => format!("v{}.{}", major, minor),
        false => extract_git_version(&pre)?,
    };

    Ok(Version {
        major,
        minor,
        git,
    })
}

fn main() {
    let version = get_version().expect("Could not extract library version");

    let sources = [
        "DeviceCommunicationExceptions.cpp",
        "NK_C_API.cc",
        "NitrokeyManager.cc",
        "command_id.cc",
        "device.cc",
        "log.cc",
        "misc.cc",
    ];
    let library_dir = format!("libnitrokey-{}", version.git);
    let library_path = path::Path::new(&library_dir);

    cc::Build::new()
        .cpp(true)
        .include(library_path.join("libnitrokey"))
        .files(sources.iter().map(|s| library_path.join(s)))
        .compile("libnitrokey.a");

    println!("cargo:rustc-link-lib=hidapi-libusb");
}
