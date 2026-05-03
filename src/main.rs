use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};

use hide_info::{mirage_tank::mirage_tank_from_bytes, HideAsImg};

#[derive(Parser)]
#[command(name = "hide_info", author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[command(rename_all = "snake_case")]
enum Commands {
    HideAsImg {
        #[command(subcommand)]
        command: HideAsImgCommand,
    },
    MirageTank {
        #[arg(long)]
        input1: PathBuf,
        #[arg(long)]
        input2: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value_t = 0.5)]
        a: f32,
        #[arg(long)]
        b: Option<f32>,
    },
}

#[derive(Subcommand)]
#[command(rename_all = "snake_case")]
enum HideAsImgCommand {
    Encode {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Decode {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::HideAsImg { command } => match command {
            HideAsImgCommand::Encode { input, output } => {
                let input_bytes = fs::read(input)?;
                let output_bytes = HideAsImg::new().encode(&input_bytes)?;
                fs::write(output, output_bytes)?;
            }
            HideAsImgCommand::Decode { input, output } => {
                let input_bytes = fs::read(input)?;
                let output_bytes = HideAsImg::new().decode(&input_bytes)?;
                fs::write(output, output_bytes)?;
            }
        },
        Commands::MirageTank {
            input1,
            input2,
            output,
            a,
            b,
        } => {
            let input1_bytes = fs::read(input1)?;
            let input2_bytes = fs::read(input2)?;
            let output_bytes = mirage_tank_from_bytes(&input1_bytes, &input2_bytes, a, b)?;
            fs::write(output, output_bytes)?;
        }
    }

    Ok(())
}
