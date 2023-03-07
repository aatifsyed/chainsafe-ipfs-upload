use chainsafe_ipfs_upload as subject;
use futures::TryStreamExt as _;
use ipfs_api_backend_hyper::{IpfsApi as _, IpfsClient, TryFromUri as _};

fn test_server() -> std::net::SocketAddr {
    get_env_as("TEST_IPFS_SERVER")
}

/// # Panics
/// - If var doesn't exist or is not unicode
/// - If var isn't parseable as T
fn get_env_as<T: std::str::FromStr>(key: &str) -> T {
    match std::env::var(key) {
        Ok(val) => match val.parse::<T>() {
            Ok(t) => t,
            Err(_) => panic!(
                "The environment variable {key}={val} couldn't be parsed as a {}",
                std::any::type_name::<T>()
            ),
        },
        Err(std::env::VarError::NotPresent) => panic!("The environment variable {key} is not set"),
        Err(std::env::VarError::NotUnicode(val)) => {
            panic!("The environment variable {key}={val:?} is not valid unicode")
        }
    }
}

#[tokio::test]
async fn upload_to_ipfs() {
    let test_data = b"hello".as_slice(); // TODO(aatifsyed): this is a good candidate for property-based testing
    let hash = dbg!(subject::upload_to_ipfs(test_server(), test_data).await).unwrap();
    let client = IpfsClient::from_socket(http::uri::Scheme::HTTP, test_server()).unwrap();
    let fetched = dbg!(
        client
            .cat(&hash)
            .map_ok(|it| it.to_vec())
            .try_concat()
            .await
    )
    .unwrap();
    assert_eq!(test_data, fetched);
}
