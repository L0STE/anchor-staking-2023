# anchor-staking-2023
A simple Staking Program for NFTs that rewards point. Written in Anchor

# Usage
Start up a test validator (NB: needs to support metaplex program to work with NFTs):

```
solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so
```

Then run:
```
anchor build
anchor deploy
```

Once deployed, ensure the program is matches in `programs/vault/lib.rs` and `Anchor.toml` and run:

```
anchor test --skip-test-validator
```
