use chainsafe_ipfs_upload::{store_string, upload_to_ipfs};
use clap::Parser as _;
use color_eyre::eyre::Context as _;
use std::{fs, io, net::SocketAddr, path::PathBuf};
use tracing::{debug, info};

#[derive(Debug, clap::Parser)]
#[clap(about, version)]
struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,

    #[arg(short, long)]
    /// The path to the file to upload to ipfs.
    /// If not supplied, stdin will be used
    file: Option<PathBuf>,

    #[arg(short, long)]
    /// socket address of ipfs API server to use (in host:port format)
    ipfs: SocketAddr,

    #[arg(short, long)]
    /// URL to ethereum API gateway
    ethereum: url::Url,

    #[arg(short, long, value_parser = parse_secret_key)]
    /// Wallet secret key.
    /// Must have enough gas to deploy contract
    // Obviously shouldn't be passed in as commandline arg
    // (other processes can snoop)
    secret_key: ethers::core::k256::SecretKey,

    #[arg(short, long, default_value_t = ethers::types::Chain::AnvilHardhat.into())]
    chain_id: u64,
}

#[test]
fn args() {
    <Args as clap::CommandFactory>::command().debug_assert();
}

// TODO(aatifsyed): test the actual command with e.g assert_cmd
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let Args {
        verbose: _,
        file,
        ipfs,
        ethereum,
        secret_key,
        chain_id,
    } = get_args_and_setup_logging()?;

    let reader: Box<dyn io::Read + Send + Sync + Unpin + 'static> = match file {
        None => Box::new(io::stdin()),
        Some(f) if f.as_os_str() == "-" => Box::new(io::stdin()),
        Some(f) => Box::new(fs::File::open(f).context("couldn't open file")?),
    };

    let cid = upload_to_ipfs(ipfs, reader)
        .await
        .context("couldn't upload to ipfs")?;

    info!(%cid, "uploaded file to ipfs");

    let address = store_string(cid, ethereum, secret_key, chain_id)
        .await
        .context("couldn't store cid in new ethereum contract")?;
    let address = format!("{address:02x}");

    info!(%address, "stored cid in new ethereum contract");

    println!("{address}");

    Ok(())
}

fn parse_secret_key(
    s: &str,
) -> Result<ethers::core::k256::SecretKey, ethers::core::k256::elliptic_curve::Error> {
    s.trim_start_matches("0x")
        .parse::<ethers::core::k256::NonZeroScalar>()
        .map(Into::into)
}

/// Parse args, gracefully exiting the process if parsing fails.
/// # Panics
/// - If global logger has already been setup
fn get_args_and_setup_logging() -> color_eyre::Result<Args> {
    color_eyre::install()?;
    let args = Args::parse();
    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive({
            // TODO(aatifsyed): default directive should be our crate
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
