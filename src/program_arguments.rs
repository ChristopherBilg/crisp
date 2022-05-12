use clap::Parser;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct ProgramArguments {
    /// Interactive mode
    #[clap(short, long)]
    pub interactive: bool,
}
