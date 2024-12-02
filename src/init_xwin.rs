use std::{path::PathBuf, process::Command};

use anyhow::Result;
use cargo_options::heading;
use clap::Parser;

use crate::common::XWinOptions;

#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    about = "Run init command",
    after_help = "",
)]
pub struct InitXWin {
    #[command(flatten)]
    common: cargo_options::CommonOptions,

    /// Path to Cargo.toml
    #[arg(long, value_name = "PATH", help_heading = heading::MANIFEST_OPTIONS)]
    pub manifest_path: Option<PathBuf>,

    #[command(flatten)]
    xwin: XWinOptions,
}

impl InitXWin {
    pub fn execute(&self) -> Result<()> {
        let mut build = Command::new("");
        self.xwin.apply_command_env(
            self.manifest_path.as_deref(),
            &self.common,
            &mut build,
        )?;
        Ok(())
    }
}
