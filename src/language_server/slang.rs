use super::LanguageServer;
use zed_extension_api::{self as zed};

#[derive(Default)]
pub struct Slang {
    cached_binary: Option<String>,
}

impl LanguageServer for Slang {
    const LANGUAGE_SERVER_ID: &'static str = "slang";
    const DOWNLOAD_REPO: &'static str = "someone13574/zed-verilog-extension";
    const DOWNLOAD_TAG: &'static str = "v0.0.12";

    fn get_cached_binary(&self) -> Option<String> {
        self.cached_binary.clone()
    }

    fn set_cached_binary(&mut self, cached_bin: Option<String>) {
        self.cached_binary = cached_bin;
    }

    fn binary_name(os: zed_extension_api::Os) -> String {
        match os {
            zed::Os::Mac | zed::Os::Linux => "slang-server",
            zed::Os::Windows => "slang-server.exe",
        }
        .to_string()
    }

    fn binary_path(
        _version: &str,
        _os: zed_extension_api::Os,
        _arch: zed_extension_api::Architecture,
    ) -> zed_extension_api::Result<String> {
        Ok("slang-server".to_string())
    }

    fn asset_name(
        _version: &str,
        os: zed_extension_api::Os,
        arch: zed_extension_api::Architecture,
    ) -> zed_extension_api::Result<String> {
        Ok(match (os, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "slang-server-aarch64-macos.tar.gz",
            // Currently not being built by CI, enable when implemented
            // (zed::Os::Mac, zed::Architecture::X8664) => "slang-server-x86_64-macos.tar.gz",
            // (zed::Os::Linux, zed::Architecture::Aarch64) => "slang-server-aarch64-linux.tar.gz",
            (zed::Os::Linux, zed::Architecture::X8664) => "slang-server-x86_64-linux.tar.gz",
            (zed::Os::Windows, zed::Architecture::X8664) => "slang-server-x86_64-windows.zip",
            (os, arch) => {
                return Err(format!("architecture {arch:?} not supported on {os:?}"));
            }
        }
        .to_string())
    }
}
