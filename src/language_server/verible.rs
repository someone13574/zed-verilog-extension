use super::LanguageServer;
use zed_extension_api::{self as zed};

#[derive(Default)]
pub struct Verible {
    cached_binary: Option<String>,
}

impl LanguageServer for Verible {
    const LANGUAGE_SERVER_ID: &'static str = "verible";
    const DOWNLOAD_REPO: &'static str = "chipsalliance/verible";
    const DOWNLOAD_TAG: &'static str = "v0.0-4023-gc1271a00";

    fn get_cached_binary(&self) -> Option<String> {
        self.cached_binary.clone()
    }

    fn set_cached_binary(&mut self, cached_bin: Option<String>) {
        self.cached_binary = cached_bin;
    }

    fn binary_name(os: zed::Os) -> String {
        match os {
            zed::Os::Mac | zed::Os::Linux => "verible-verilog-ls",
            zed::Os::Windows => "verible-verilog-ls.exe",
        }
        .to_string()
    }

    fn binary_path(version: &str, os: zed::Os, _arch: zed::Architecture) -> zed::Result<String> {
        let dir = format!(
            "verible-{version}{}",
            match os {
                zed::Os::Mac => "-macOS/bin",
                zed::Os::Linux => "/bin",
                zed::Os::Windows => "-win64/",
            }
        );
        let binary_name = Self::binary_name(os);

        Ok(format!("{dir}/{binary_name}"))
    }

    fn asset_name(version: &str, os: zed::Os, arch: zed::Architecture) -> zed::Result<String> {
        let suffix = match (os, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64)
            | (zed::Os::Mac, zed::Architecture::X8664) => "macOS.tar.gz",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "linux-static-arm64.tar.gz",
            (zed::Os::Linux, zed::Architecture::X8664) => "linux-static-x86_64.tar.gz",
            (zed::Os::Windows, zed::Architecture::X8664) => "win64.zip",
            _ => {
                return Err(format!("architecture {arch:?} not supported on {os:?}"));
            }
        };

        Ok(format!("verible-{version}-{suffix}"))
    }
}
