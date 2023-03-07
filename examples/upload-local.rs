/// Example binary as I try and get to grips with this library
use clap::Parser as _;
use futures::TryStreamExt as _;
use http::uri::Scheme;
use ipfs_api_backend_hyper::{response::AddResponse, IpfsApi as _, IpfsClient, TryFromUri as _};
use std::net::SocketAddrV4;

#[derive(Debug, clap::Parser)]
#[clap(about = "Example binary as I try and get to grips with IPFS.
Requires kubo to be running: `ipfs daemon --init --init-profile=test`")]
struct Args {
    /// API server listening on /ip4/<addr>/tcp/<port>
    #[arg(short, long)]
    addr: SocketAddrV4,
    /// Data to upload
    #[arg(short, long, default_value = "hello, world!")]
    data: String,
}

#[tokio::main]
async fn main() {
    let args = dbg!(Args::parse());
    let data = &*args.data.into_bytes().leak();
    let client = IpfsClient::from_ipv4(Scheme::HTTP, args.addr).expect("couldn't create client");
    let AddResponse {
        name: _,
        hash,
        size: _,
    } = dbg!(client.add(data).await).expect("couldn't add file");
    let retrieved = dbg!(
        client
            .cat(&hash)
            .map_ok(|it| it.to_vec())
            .try_concat()
            .await
    )
    .expect("couldn't get file");
    assert_eq!(retrieved, data, "uploaded and fetched data are different");
}
