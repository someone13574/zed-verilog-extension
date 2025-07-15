use super::LanguageServer;
use std::fs;
use zed_extension_api::{self as zed};

#[derive(Default)]
pub struct Veridian {
    cached_binary: Option<String>,
}

impl LanguageServer for Veridian {
    const LANGUAGE_SERVER_ID: &'static str = "veridian";
    const DOWNLOAD_REPO: &'static str = "someone13574/zed-verilog-extension";
    const DOWNLOAD_TAG: &'static str = "v0.0.9";

    fn binary_name(os: zed_extension_api::Os) -> String {
        match os {
            zed::Os::Mac | zed::Os::Linux => "veridian",
            zed::Os::Windows => "verible-verilog-ls.exe",
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
            (zed::Os::Linux, zed::Architecture::Aarch64) => "veridian-aarch64-linux-gnu.tar.gz",
            (zed::Os::Linux, zed::Architecture::X8664) => "veridian-x86_64-linux-gnu.tar.gz",
            (zed::Os::Windows, zed::Architecture::X8664) => "veridian-x86_64-windows-mscv.zip",
            (os, arch) => {
                return Err(format!("architecture {arch:?} not supported on {os:?}"));
            }
        }
        .to_string())
    }

    fn download_binary(
        &self,
        language_server_id: &zed_extension_api::LanguageServerId,
        os: zed_extension_api::Os,
        arch: zed_extension_api::Architecture,
    ) -> zed_extension_api::Result<String> {
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

            zed::download_file(
                &asset.download_url,
                "",
                match os {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|err| format!("failed to download file `{}`: {err}", asset.download_url))?;
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(binary_path)
    }

    fn get_cached_binary(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<String> {
        if let Some(path) = &self.cached_binary {
            if !fs::metadata(path).is_ok_and(|metadata| metadata.is_file()) {
                self.cached_binary = None;
            } else {
                return Ok(path.to_string());
            }
        }

        let (os, arch) = zed::current_platform();
        if let Some(path) = worktree.which(&Self::binary_name(os)) {
            self.cached_binary = Some(path);
        } else {
            self.cached_binary = Some(self.download_binary(language_server_id, os, arch)?);
        }

        Ok(self.cached_binary.clone().unwrap())
    }
}
