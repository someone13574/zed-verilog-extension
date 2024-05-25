use std::fs;
use zed::{LanguageServerId, Worktree};
use zed_extension_api as zed;

mod platform;

struct VerilogExtension {
    cached_binary_path: Option<String>,
}

impl VerilogExtension {
    const LANGUAGE_SERVER_ID: &'static str = "veridian";

    fn search_for_varidian(&self, worktree: &zed::Worktree) -> Option<String> {
        if let Some(path) = worktree.which(&platform::binary_name()) {
            return Some(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |metadata| metadata.is_file()) {
                return Some(path.clone());
            }
        }

        None
    }

    fn get_veridian_asset(
        &self,
        language_server_id: &LanguageServerId,
    ) -> zed::Result<(zed::GithubReleaseAsset, String)> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "someone13574/zed-verilog-extension",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset_name = platform::veridian_asset_name()?;
        let asset = release
            .assets
            .into_iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching `{asset_name}`"))?;

        Ok((asset, release.version))
    }

    fn download_veridian(&mut self, language_server_id: &LanguageServerId) -> zed::Result<String> {
        let (asset, version) = self.get_veridian_asset(language_server_id)?;
        let binary_path = format!("{version}/{}", platform::binary_name());

        if !fs::metadata(&binary_path).map_or(false, |metadata| metadata.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version,
                platform::veridian_asset_file_type(),
            )
            .map_err(|err| format!("failed to download file `{}`: {err}", asset.download_url))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                println!("{:?}", entry.unwrap().path());
            }

            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::None,
            );
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for VerilogExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        if language_server_id.as_ref() != Self::LANGUAGE_SERVER_ID {
            return Err(format!(
                "unknown language server: {}",
                language_server_id.as_ref()
            ));
        }

        let veridian_binary = if let Some(binary) = self.search_for_varidian(worktree) {
            binary
        } else {
            self.download_veridian(language_server_id)?
        };

        Ok(zed::Command {
            command: veridian_binary,
            args: Vec::new(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(VerilogExtension);
