mod analysis;
mod cli;
mod commands;
mod input;
mod output;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        cli::Commands::Stats { input, recursive } => {
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table = commands::stats::run(text.as_str()?, &name)?;
                print!("{}", table.render(&cli.format, cli.no_color)?);
            }
        }
        cli::Commands::Ngrams {
            input,
            n,
            top,
            min_freq,
            case_insensitive,
            recursive,
        } => {
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table = commands::ngrams::run(
                    text.as_str()?,
                    &name,
                    *n,
                    *top,
                    *min_freq,
                    *case_insensitive,
                )?;
                print!("{}", table.render(&cli.format, cli.no_color)?);
            }
        }
        cli::Commands::Tokens { input, recursive } => {
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table = commands::tokens::run(text.as_str()?, &name)?;
                print!("{}", table.render(&cli.format, cli.no_color)?);
            }
        }
    }

    Ok(())
}
