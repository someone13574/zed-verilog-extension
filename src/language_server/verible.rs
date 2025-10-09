use super::LanguageServer;
use std::fs;
use zed_extension_api::{self as zed};

#[derive(Default)]
pub struct Verible {
    cached_binary: Option<String>,
}

impl LanguageServer for Verible {
    const LANGUAGE_SERVER_ID: &'static str = "verible";
    const DOWNLOAD_REPO: &'static str = "chipsalliance/verible";
    const DOWNLOAD_TAG: &'static str = "v0.0-4023-gc1271a00";

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
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
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
