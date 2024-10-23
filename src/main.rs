use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;
use std::io;

use types::Context;

mod list;
mod redfish;
mod types;
mod view;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "xpuctl")]
#[command(author = "Klaus Ma <klaus1982.cn@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "XPU command line", long_about = None)]
struct Args {
    #[clap(flatten)]
    options: Options,

    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
    #[arg(long = "config-file", short = 'c', default_value_t=("~/.xpuctl").to_string())]
    config_file: String,
}

#[derive(Debug, Subcommand, Clone)]
enum SubCommand {
    /// List all XPUs
    List,
    /// View the detail of XPU
    View {
        #[arg(long = "xpu", short = 'x')]
        xpu: usize,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.options.config_file).expect(
        format!(
            "Failed to read configuration file <{}>.",
            &args.options.config_file
        )
        .as_str(),
    );

    let cxt: Context = toml::from_str(&contents).expect(
        format!(
            "Failed to parse configuration file <{}>.",
            &args.options.config_file
        )
        .as_str(),
    );

    match &args.subcommand {
        SubCommand::List => list::run(&cxt).await?,
        SubCommand::View { xpu } => view::run(&cxt, *xpu).await?,
    }

    Ok(())
}
