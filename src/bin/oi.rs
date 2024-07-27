use clap::{Parser, Subcommand};
use oi_utility::commands::*;

#[derive(Parser)]
struct CliArgs {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run(run::RunArgs),
    New(new::NewArgs),
    Check(check::CheckArgs),
    Store(store::StoreArgs),
}

fn main() -> eyre::Result<()> {
    let args = CliArgs::parse();

    match args.commands {
        Commands::New(args) => new::new(args)?,
        Commands::Run(args) => run::run(args)?,
        Commands::Check(args) => check::check(args)?,
        Commands::Store(args) => store::store(args)?,
    }

    Ok(())
}
