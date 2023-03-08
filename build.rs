use std::{fs::File, path::Path};

use color_eyre::eyre::{bail, Context as _, ContextCompat};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let contract_path = "contracts/StoredString.sol";
    cargo_emit::rerun_if_changed!(contract_path);
    let contract = Path::new(env!("CARGO_MANIFEST_DIR")).join(contract_path);
    // Compile the contract
    let (Some(abi), Some(bytecode), _runtime_bytecode) = ethers::solc::Solc::default()
    .compile_source(contract)
    .context("couldn't compile contract")?
    .find("StoredString")
    .context("couldn't find StoredString contract")?.into_parts() else {
        bail!("contract compilation did not return abi or bytecode")
    };

    // Pass the compiled artifacts to runtime
    let out_dir = std::env::var_os("OUT_DIR").expect("build script is always run with OUT_DIR set");
    let out_dir = Path::new(&out_dir);

    serde_json::to_writer(
        File::options()
            .write(true)
            .create(true)
            .open(out_dir.join("abi.json"))
            .context("unable to open abi.json")?,
        &abi,
    )
    .context("couldn't write abi.json")?;

    std::fs::write(out_dir.join("bytecode.bin"), bytecode)
        .context("couldn't write bytecode.bin")?;
    Ok(())
}
