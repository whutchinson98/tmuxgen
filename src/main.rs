use args::Args;
use clap::Parser;
use config::Config;
use tracing_subscriber::EnvFilter;

mod args;
mod config;
mod constants;

fn main() -> anyhow::Result<()> {
    // setup tracing if RUST_LOG is set
    if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::default())
            .init();
    }

    let args = Args::parse();

    tracing::trace!(args=?args, "args parsed");

    Config::from_file(args.tmuxgen_file.as_deref())?;

    // load in the tmuxgen.toml file
    Ok(())
}
