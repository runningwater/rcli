/// rcli csv -i input.csv -o output.json --header -d ','
/// rcli genpass -l 32
///
/// rcli base64 encode
/// rcli base64 encode --format urlsafe
/// rcli base64 encode --format urlsafe -i Cargo.toml
use clap::Parser;
use rcli::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();

    opts.cmd.execute().await?;

    Ok(())
}
