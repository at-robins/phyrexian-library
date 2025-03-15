use clap::{Parser, Subcommand};

/// The "Phyrexian Library" is a tool for organising trading card collections.
#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "at-robins")]
pub struct CommandLineArguments {
    #[command(subcommand)]
    sub_commad: SubCommand,
}

impl CommandLineArguments {
    pub fn sub_command(&self) -> &SubCommand {
        &self.sub_commad
    }
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[clap(version = "0.1", author = "at-robins")]
    Magic(Magic),
}

/// The "Phyrexian Library" is a tool for organising trading card collections.
#[derive(Parser, Debug)]
pub struct Magic {
    /// Launches the graphical user interface.
    #[clap(short, long)]
    gui: bool,
}

impl Magic {
    pub fn gui(&self) -> bool {
        self.gui
    }
}
