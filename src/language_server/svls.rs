use super::LanguageServer;
use zed_extension_api::{self as zed};

#[derive(Default)]
pub struct Svls {
    cached_binary: Option<String>,
}

impl LanguageServer for Svls {
    const LANGUAGE_SERVER_ID: &'static str = "svls";
    const DOWNLOAD_REPO: &'static str = "dalance/svls";
    const DOWNLOAD_TAG: &'static str = "v0.2.14";

    fn get_cached_binary(&self) -> Option<String> {
        self.cached_binary.clone()
    }

    fn set_cached_binary(&mut self, cached_bin: Option<String>) {
        self.cached_binary = cached_bin;
    }

    fn binary_name(os: zed_extension_api::Os) -> String {
        match os {
            zed::Os::Mac | zed::Os::Linux => "svls",
            zed::Os::Windows => "svls.exe",
        }
        .to_string()
    }

    fn binary_path(
        _version: &str,
        os: zed_extension_api::Os,
        _arch: zed_extension_api::Architecture,
    ) -> zed_extension_api::Result<String> {
        Ok(match os {
            zed_extension_api::Os::Windows => "target/x86_64-pc-windows-msvc/release/svls.exe",
            _ => "svls",
        }
        .to_string())
    }

    fn asset_name(
        version: &str,
        os: zed_extension_api::Os,
        arch: zed_extension_api::Architecture,
    ) -> zed_extension_api::Result<String> {
        let target = match (os, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "aarch64-mac",
            (zed::Os::Mac, zed::Architecture::X8664) => "x86_64-mac",
            (zed::Os::Linux, zed::Architecture::X8664) => "x86_64-lnx",
            (zed::Os::Windows, zed::Architecture::X8664) => "x86_64-win",
            (os, arch) => return Err(format!("architecture {arch:?} not supported on {os:?}")),
        };
        Ok(format!("svls-{version}-{target}.zip"))
    }

    fn asset_type(
        _os: zed_extension_api::Os,
    ) -> zed_extension_api::Result<zed_extension_api::DownloadedFileType> {
        Ok(zed_extension_api::DownloadedFileType::Zip)
    }
}
