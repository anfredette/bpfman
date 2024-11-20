// SPDX-License-Identifier: Apache-2.0
// Copyright Authors of bpfman

use anyhow::anyhow;
use args::Commands;
use attach::execute_attach;
use clap::Parser;
use detach::execute_detach;
use get::execute_get;
use list::execute_list;
use log::debug;
use unload::execute_unload;

mod args;
mod attach;
mod completions;
mod detach;
mod get;
mod image;
mod list;
mod load;
mod manpage;
mod table;
mod unload;

fn main() -> anyhow::Result<()> {
    env_logger::try_init()?;
    debug!("Log using env_logger");

    let cli = crate::args::Cli::parse();

    cli.command.execute()
}

impl Commands {
    pub(crate) fn execute(&self) -> Result<(), anyhow::Error> {
        match self {
            Commands::Load(l) => l.execute(),
            Commands::Unload(args) => execute_unload(args),
            Commands::Attach(args) => execute_attach(args),
            Commands::Detach(args) => execute_detach(args),
            Commands::List(args) => execute_list(args),
            Commands::Get(args) => execute_get(args).map_err(|e| anyhow!("get error: {e}")),
            Commands::Image(i) => i.execute(),
            Commands::Man(args) => manpage::generate(args),
            Commands::Completions(args) => completions::generate(args),
        }?;

        Ok(())
    }
}
