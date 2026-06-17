use std::io;

use clap::{Args, CommandFactory};
use clap_complete::{Shell, generate};

use crate::Cli;

/// Generate shell completions, printing to stdout for you to redirect to a file. Ignores dry run mode.
#[derive(Args)]
pub struct CompletionsCommand {
    /// Shell to generate completions for.
    #[arg(value_enum)]
    pub shell: Shell,
}

impl CompletionsCommand {
    pub fn execute(self) {
        eprintln!("Generating completions for {:?}...", self.shell);

        let mut cmd = Cli::command();
        let bin_name = cmd.get_name().to_owned();
        generate(self.shell, &mut cmd, bin_name, &mut io::stdout());
    }
}
