extern crate cc;

use std::path::Path;

fn main() {
    let sources = [
        "DeviceCommunicationExceptions.cpp",
        "NK_C_API.cc",
        "NitrokeyManager.cc",
        "command_id.cc",
        "device.cc",
        "log.cc",
        "misc.cc",
    ];
    let library_dir = Path::new("libnitrokey-3.3");

    cc::Build::new()
        .cpp(true)
        .include(library_dir.join("libnitrokey"))
        .files(sources.iter().map(|s| library_dir.join(s)))
        .compile("libnitrokey.a");

    println!("cargo:rustc-link-lib=hidapi-libusb");
}
