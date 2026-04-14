use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = abbotik_cli::cli::Cli::parse();
    abbotik_cli::commands::run(cli).await
}
