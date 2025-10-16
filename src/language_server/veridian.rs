use super::LanguageServer;
use zed_extension_api::{self as zed};

#[derive(Default)]
pub struct Veridian {
    cached_binary: Option<String>,
}

impl LanguageServer for Veridian {
    const LANGUAGE_SERVER_ID: &'static str = "veridian";
    const DOWNLOAD_REPO: &'static str = "someone13574/zed-verilog-extension";
    const DOWNLOAD_TAG: &'static str = "v0.0.13";

    fn get_cached_binary(&self) -> Option<String> {
        self.cached_binary.clone()
    }

    fn set_cached_binary(&mut self, cached_bin: Option<String>) {
        self.cached_binary = cached_bin;
    }

    fn binary_name(os: zed_extension_api::Os) -> String {
        match os {
            zed::Os::Mac | zed::Os::Linux => "veridian",
            zed::Os::Windows => "veridian.exe",
        }
        .to_string()
    }

    fn binary_path(
        _version: &str,
        _os: zed_extension_api::Os,
        _arch: zed_extension_api::Architecture,
    ) -> zed_extension_api::Result<String> {
        Ok("veridian".to_string())
    }

    fn asset_name(
        _version: &str,
        os: zed_extension_api::Os,
        arch: zed_extension_api::Architecture,
    ) -> zed_extension_api::Result<String> {
        Ok(match (os, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "veridian-aarch64-macos.tar.gz",
            (zed::Os::Mac, zed::Architecture::X8664) => "veridian-aarch64-x86_64-macos.tar.gz",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "veridian-aarch64-linux-musl.tar.gz",
            (zed::Os::Linux, zed::Architecture::X8664) => "veridian-x86_64-linux-musl.tar.gz",
            (zed::Os::Windows, zed::Architecture::X8664) => "veridian-x86_64-windows-mscv.zip",
            (os, arch) => {
                return Err(format!("architecture {arch:?} not supported on {os:?}"));
            }
        }
        .to_string())
    }
}
