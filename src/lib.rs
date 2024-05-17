use std::fs;

use zed::{LanguageServerId, Worktree};
use zed_extension_api as zed;

struct VerilogExtension {
    cached_binary_path: Option<String>,
}

impl VerilogExtension {
    const LANGUAGE_SERVER_ID: &'static str = "verible-ls";

    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        if let Some(path) = worktree.which("verible-verilog-ls") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let (asset, version) = self.get_binary_asset()?;

        let (platform, _) = zed::current_platform();
        let (suffix, binary_path) = match platform {
            zed::Os::Mac => ("-macOS", "bin/verible-verilog-ls"),
            zed::Os::Linux => ("", "bin/verible-verilog-ls"),
            zed::Os::Windows => ("-win64", "verible-verilog-ls.exe"),
        };
        let version_dir = format!("verible-{version}{suffix}");
        let binary_path = format!("{version_dir}/{binary_path}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                "",
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    println!("removing: {:?}", entry.file_name());
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn get_binary_asset(&self) -> zed::Result<(zed::GithubReleaseAsset, String)> {
        let release = zed::latest_github_release(
            "chipsalliance/verible",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (os, arch) = zed::current_platform();
        let (asset_suffix, tar_gz) = match (os, arch) {
            (zed::Os::Mac, zed::Architecture::X8664)
            | (zed::Os::Mac, zed::Architecture::Aarch64) => ("macOS", true),
            (zed::Os::Linux, zed::Architecture::Aarch64) => ("linux-static-arm64", true),
            (zed::Os::Linux, zed::Architecture::X8664) => ("linux-static-x86_64", true),
            (zed::Os::Windows, zed::Architecture::X8664) => ("win64", false),
            _ => {
                return Err(format!(
                    "Architecture {arch:?} not supported on {os:?} with the Verible language server"
                ))
            }
        };
        let asset_name = format!(
            "verible-{}-{}.{}",
            release.version,
            asset_suffix,
            if tar_gz { "tar.gz" } else { "zip" }
        );
        Ok((
            release
                .assets
                .into_iter()
                .find(|asset| asset.name == asset_name)
                .ok_or_else(|| format!("no asset found matching `{asset_name}`"))?,
            release.version,
        ))
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

        let language_settings =
            zed::settings::LanguageSettings::for_worktree(Some("Verilog"), worktree)?;
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![
                "--indentation_spaces".to_string(),
                language_settings.tab_size.to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(VerilogExtension);
