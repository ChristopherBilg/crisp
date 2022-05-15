use clap::Parser;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct ProgramArguments {
    /// Interactive mode
    #[clap(short, long)]
    pub interactive: bool,

    /// Filename mode
    #[clap(short, long)]
    pub filename: Option<String>,
}
