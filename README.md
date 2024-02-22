# core-mx-deputy-appointer-interface-sc

A sample deputy appointer interface smart contract that you can use for the data marshal network to integrate with. This 3rd party Appointer Smart Contract exposes a public method called viewDeputyAddress(), the data marshal uses this to allow the deputy to access Data NFTs stored inside this smart contract.

## How can you use/test it on devnet?

- Deploy this smart contract
- Call `initializeContract(deputyAddress)` or `setDeputyAddress(deputyAddress)` to set the deputy
- Deposit a Data NFT into this smart contract (e.g. https://devnet-explorer.multiversx.com/nfts/DATANFTFT-e0b917-40)
- Use the Data Marshal's `access` route via the `Data NFT SDK`, where the deputyAddress can send in a native auth token and request to open the `DATANFTFT-e0b917-40` Data NFT
- It should work!

## Dev env

- Uses `multiversx-sc-* 0.45.1`
- Packed using **mxpy 9.5.1**. Check version using `mxpy --version`
- Built on Rust version `1.76.0-nightly`. Check your Rust version by running `rustc --version`. To update your Rust, run `rustup update`. To set to nightly run `rustup default nightly`
- After you make sure you have the minimum Rust version you can then begin development. After you clone repo and before you run build, deploy or run the tests - follow these steps (most likely only needed the 1st time)
- [Upgrades] Note that when we upgrade smart contract, we should again follow the steps below too as lib version may have changed (but for upgrade I skipped the rustup default nightly cmd and did the others)

```
rustup default nightly
mxpy deps install rust --overwrite
cargo clean
cargo build
```

- The above should all work without any errors, next you can successfully run the following command to build via mxpy: `mxpy contract build`
- mxpy may ask you to install `nodejs` and `wasm-opt` to optimize the build, if so then follow instructions given by mxpy and do this
- You can now run the tests. See "How to test" section below
- You can now update code as needed

### How to test

The tests are located in the tests folder, in the rust_tests file. In order to run the tests one can use the command:

```shell
    cargo test --package deputyappointerinterface --test rust_tests --  --nocapture
```

### How to deploy to devnet

```shell
    source interaction/devnet.snippets.sh
```

After using that, to deploy one can simply use:

```shell
    deploy
```

### How to interact

After deployment, one can interact with the smart contract and test its functionality. To do so, one can use the interaction snippets already presented above. More explanations can be found about the snippets inside the devnet.snippets file.
