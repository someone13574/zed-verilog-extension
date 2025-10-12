pub mod slang;
pub mod verible;
pub mod veridian;

use std::fs;
use zed_extension_api::{self as zed};

pub trait LanguageServer {
    const LANGUAGE_SERVER_ID: &'static str;
    const DOWNLOAD_REPO: &'static str;
    const DOWNLOAD_TAG: &'static str;

    fn get_cached_binary(&self) -> Option<String>;
    fn set_cached_binary(&mut self, cached_bin: Option<String>);

    fn binary_name(os: zed::Os) -> String;
    fn binary_path(version: &str, os: zed::Os, arch: zed::Architecture) -> zed::Result<String>;
    fn asset_name(version: &str, os: zed::Os, arch: zed::Architecture) -> zed::Result<String>;

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

    fn get_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        if let Some(path) = &self.get_cached_binary() {
            if !fs::metadata(path).is_ok_and(|metadata| metadata.is_file()) {
                self.set_cached_binary(None);
            } else {
                return Ok(path.to_string());
            }
        }

        let (os, arch) = zed::current_platform();
        // Try to locate binary in $PATH
        if let Some(path) = worktree.which(&Self::binary_name(os)) {
            self.set_cached_binary(Some(path));
        } else {
            // Download if not found
            self.set_cached_binary(Some(self.download_binary(language_server_id, os, arch)?));
        }

        Ok(self.get_cached_binary().unwrap())
    }
}
