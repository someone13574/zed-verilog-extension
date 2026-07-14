use super::LanguageServer;
use std::fs;
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
        _os: zed_extension_api::Os,
        _arch: zed_extension_api::Architecture,
    ) -> zed_extension_api::Result<String> {
        Ok("svls".to_string())
    }

    fn asset_name(
        version: &str,
        os: zed_extension_api::Os,
        arch: zed_extension_api::Architecture,
    ) -> zed_extension_api::Result<String> {
        Ok(match (os, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => {
                format!("svls-{version}-aarch64-mac.zip")
            }
            (zed::Os::Mac, zed::Architecture::X8664) => {
                format!("svls-{version}-x86_64-mac.zip")
            }
            (zed::Os::Linux, zed::Architecture::X8664) => {
                format!("svls-{version}-x86_64-lnx.zip")
            }
            (zed::Os::Windows, zed::Architecture::X8664) => {
                format!("svls-{version}-x86_64-win.zip")
            }
            (os, arch) => {
                return Err(format!("architecture {arch:?} not supported on {os:?}"));
            }
        })
    }

    // svls releases use zip for all platforms, unlike the default which uses GzipTar for Mac/Linux
    fn download_binary(
        &self,
        language_server_id: &zed::LanguageServerId,
        os: zed::Os,
        arch: zed::Architecture,
    ) -> zed::Result<String> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::github_release_by_tag_name(Self::DOWNLOAD_REPO, Self::DOWNLOAD_TAG)?;

        let asset_name = Self::asset_name(&release.version, os, arch)?;
        let asset = release
            .assets
            .into_iter()
            .find(|asset| asset.name == asset_name)
            .ok_or(format!("no asset found matching `{asset_name}`"))?;
        let binary_path = Self::binary_path(&release.version, os, arch)?;

        if !fs::metadata(&binary_path).is_ok_and(|metadata| metadata.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(&asset.download_url, "", zed::DownloadedFileType::Zip).map_err(
                |err| format!("failed to download file `{}`: {err}", asset.download_url),
            )?;
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(binary_path)
    }
}
