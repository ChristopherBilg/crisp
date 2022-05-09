use clap::Parser;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct ProgramArguments {
    /// Interactive mode
    #[clap(short, long)]
    pub interactive: bool,

    /// Command-line mode
    #[clap(short, long)]
    pub command_line: Option<String>,

    /// File-input mode
    #[clap(short, long)]
    pub filename: Option<String>,
}
