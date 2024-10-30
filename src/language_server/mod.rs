pub mod verible;
pub mod veridian;

use zed_extension_api::{self as zed};

pub trait LanguageServer {
    const LANGUAGE_SERVER_ID: &'static str;
    const DOWNLOAD_REPO: &'static str;
    const DOWNLOAD_TAG: &'static str;

    fn binary_name(os: zed::Os) -> String;
    fn binary_path(version: &str, os: zed::Os, arch: zed::Architecture) -> zed::Result<String>;
    fn asset_name(version: &str, os: zed::Os, arch: zed::Architecture) -> zed::Result<String>;

    fn download_binary(
        &self,
        language_server_id: &zed::LanguageServerId,
        os: zed::Os,
        arch: zed::Architecture,
    ) -> zed::Result<String>;
    fn get_cached_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String>;
}
