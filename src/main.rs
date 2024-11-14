use clap::{Parser, Subcommand};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use tracing_subscriber::{filter::EnvFilter, filter::LevelFilter, fmt, prelude::*};

use types::Context;

mod discover;
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
    /// Discover all XPUs
    Discover,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
        .add_directive("tower=warn".parse()?)
        .add_directive("rustls=warn".parse()?)
        .add_directive("reqwest=info".parse()?)
        .add_directive("hyper=info".parse()?)
        .add_directive("h2=warn".parse()?);
    // if config.debug != 0 {
    //     env_filter = env_filter.add_directive(
    //         match config.debug {
    //             1 => LevelFilter::DEBUG,
    //             _ => LevelFilter::TRACE,
    //         }
    //         .into(),
    //     );
    // }

    tracing_subscriber::registry()
        .with(fmt::Layer::default().compact().with_writer(std::io::stderr))
        .with(env_filter)
        .try_init()?;

    let args = Args::parse();
    use std::path::Path;

    let path = Path::new(&args.options.config_file);
    let mut config_file = PathBuf::from("/");

    for i in path.iter() {
        if i == "~" {
            config_file = config_file.join(env::var("HOME")?);
        } else {
            config_file = config_file.join(i);
        }
    }

    let contents = fs::read_to_string(&config_file.to_str().unwrap()).expect(
        format!(
            "Failed to read configuration file <{}>.",
            &args.options.config_file
        )
        .as_str(),
    );

    let mut cxt: Context = toml::from_str(&contents).expect(
        format!(
            "Failed to parse configuration file <{}>.",
            &args.options.config_file
        )
        .as_str(),
    );

    for bmc in cxt.bmc.iter_mut() {
        if bmc.password.is_none() {
            bmc.password = Some(cxt.password.clone());
        }

        if bmc.username.is_none() {
            bmc.username = Some(cxt.username.clone());
        }
    }

    match &args.subcommand {
        SubCommand::Discover => discover::run(&cxt).await?,
        SubCommand::List => list::run(&cxt).await?,
        SubCommand::View { xpu } => view::run(&cxt, *xpu).await?,
    }

    Ok(())
}
