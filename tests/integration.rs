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

#[test]
fn upload_to_ipfs() {}
