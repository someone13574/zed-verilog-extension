use language_server::{slang::Slang, verible::Verible, veridian::Veridian, LanguageServer};

use zed::{LanguageServerId, Worktree};
use zed_extension_api::{self as zed};

mod language_server;

struct VerilogExtension {
    verible: Verible,
    veridian: Veridian,
    slang: Slang,
}

impl zed::Extension for VerilogExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            verible: Default::default(),
            veridian: Default::default(),
            slang: Default::default(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        match language_server_id.as_ref() {
            "slang" => Ok(zed::Command {
                command: self.slang.get_binary(language_server_id, worktree)?,
                args: Vec::new(),
                env: Vec::new(),
            }),
            Veridian::LANGUAGE_SERVER_ID => Ok(zed::Command {
                command: self.veridian.get_binary(language_server_id, worktree)?,
                args: Vec::new(),
                env: Vec::new(),
            }),
            Verible::LANGUAGE_SERVER_ID => {
                let language_settings =
                    zed::settings::LanguageSettings::for_worktree(Some("Verilog"), worktree)?;

                Ok(zed::Command {
                    command: self.verible.get_binary(language_server_id, worktree)?,
                    args: vec![
                        "--indentation_spaces".to_string(),
                        language_settings.tab_size.to_string(),
                    ],
                    env: Vec::new(),
                })
            }
            id => Err(format!("unknown language server `{id}`"))?,
        }
    }
}

zed::register_extension!(VerilogExtension);
