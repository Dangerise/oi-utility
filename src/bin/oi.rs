use std::path::PathBuf;

use clap::{Parser, Subcommand};
use oi_utility::{commands::*, workspace::Workspace};

#[derive(Parser)]
struct CliArgs {
    #[command(subcommand)]
    commands: Commands,
    #[arg(short, long)]
    main: Option<PathBuf>,
    #[arg(short, long)]
    std: Option<PathBuf>,
    #[arg(short, long)]
    gen: Option<PathBuf>,
    #[arg(short,long,default_value_t=String::from("./"))]
    path: String,
}

#[derive(Subcommand)]
enum Commands {
    Run(run::RunArgs),
    New(new::NewArgs),
    Check(check::CheckArgs),
    Store(store::StoreArgs),
}

fn main() -> eyre::Result<()> {
    let CliArgs {
        commands,
        main,
        std,
        gen,
        path,
    } = CliArgs::parse();

    let mut workspace = Workspace::new(path)?;

    if let Some(solution) = main {
        workspace.main_code.replace(solution);
    }
    if let Some(std) = std {
        workspace.std_code.replace(std);
    }
    if let Some(gen) = gen {
        workspace.generator_code.replace(gen);
    }

    match commands {
        Commands::New(args) => new::new(args)?,
        Commands::Run(args) => run::run(workspace, args)?,
        Commands::Check(args) => check::check(workspace, args)?,
        Commands::Store(args) => store::store(args)?,
    }

    Ok(())
}
