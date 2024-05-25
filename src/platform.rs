use zed_extension_api as zed;

pub fn binary_name() -> String {
    match zed::current_platform().0 {
        zed::Os::Mac | zed::Os::Linux => "veridian",
        zed::Os::Windows => "veridian.exe",
    }
    .to_string()
}

pub fn veridian_asset_name() -> zed::Result<String> {
    Ok(match zed::current_platform() {
        (zed::Os::Mac, zed::Architecture::Aarch64) => "veridian-aarch64-macos.tar.gz",
        (zed::Os::Mac, zed::Architecture::X8664) => "veridian-aarch64-x86_64-macos.tar.gz",
        (zed::Os::Linux, zed::Architecture::Aarch64) => "veridian-aarch64-linux-gnu.tar.gz",
        (zed::Os::Linux, zed::Architecture::X8664) => "veridian-x86_64-linux-gnu.tar.gz",
        (zed::Os::Windows, zed::Architecture::X8664) => "veridian-x86_64-windows-mscv.zip",
        (os, arch) => {
            return Err(format!("Architecture {arch:?} not supported on {os:?}"));
        }
    }
    .to_string())
}

pub fn veridian_asset_file_type() -> zed::DownloadedFileType {
    match zed::current_platform().0 {
        zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
        zed::Os::Windows => zed::DownloadedFileType::Zip,
    }
}
