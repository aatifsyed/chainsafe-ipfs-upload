use clap::Parser as _;
use color_eyre::eyre::Context as _;
use ipfs_api_backend_hyper::{response::AddResponse, IpfsApi as _, IpfsClient, TryFromUri as _};
use std::{fs, io, path::PathBuf};
use tracing::{debug, info};

#[derive(Debug, clap::Parser)]
#[clap(about, version)]
struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,

    #[arg(short, long)]
    /// The path to the file to upload to ipfs.
    file: Option<PathBuf>,

    #[arg(short, long, default_value = "http://ipfs.io")]
    /// URI of ipfs server to use
    server: String,
}

#[test]
fn args() {
    <Args as clap::CommandFactory>::command().debug_assert();
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = get_args_and_setup_logging()?;

    // establish a connection
    let client = IpfsClient::from_str(&args.server)?;
    let version = client
        .version()
        .await
        .context("couldn't get server ipfs version")?;
    debug!(?version, "connected to ipfs server");

    let AddResponse { name, hash, size } = match args.file {
        None => client.add(io::stdin()).await,
        Some(file) if file.as_os_str() == "-" => client.add(io::stdin()).await,
        Some(file) => {
            client
                .add(fs::File::open(file).context("couldn't read input file")?)
                .await
        }
    }
    .context("couldn't upload file to ipfs")?;

    info!(%name, %hash, %size, "uploaded file to ipfs");

    Ok(())
}

/// Parse args, gracefully exiting the process if parsing fails.
/// # Panics
/// - If global logger has already been setup
fn get_args_and_setup_logging() -> color_eyre::Result<Args> {
    color_eyre::install()?;
    let args = Args::parse();
    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive({
            use tracing_subscriber::filter::LevelFilter;
            match args.verbose.log_level() {
                Some(log::Level::Error) => LevelFilter::ERROR,
                Some(log::Level::Warn) => LevelFilter::WARN,
                Some(log::Level::Info) => LevelFilter::INFO,
                Some(log::Level::Debug) => LevelFilter::DEBUG,
                Some(log::Level::Trace) => LevelFilter::TRACE,
                None => LevelFilter::OFF,
            }
            .into()
        })
        .from_env()
        .context("couldn't parse RUST_LOG environment variable")?;
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(io::stderr)
        .init();
    debug!(?args);
    Ok(args)
}
