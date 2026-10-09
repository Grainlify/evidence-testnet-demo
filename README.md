# evidence-testnet-demo

A tiny Soroban contract (a guestbook) deployed on **Stellar testnet**. It exists only as a test project for Grainlify's evidence engine: the engine checks this repo, the deployed contract and its usage, the same way it checks a real grant project.

- Contract: `contracts/guestbook` — `init(admin)` (admin only), `sign(from, msg)` (emits a `signed` event), `count()`, `signatures_of(who)`.
- Testnet contract v1: `CDKB4W4H2UQAVRYV5HCPTGOU4FYCKJNIHHIEUP7BR6KBHJYLZH4BZRJ5`
- Testnet contract v2 (last message, upgradable): `CBME3COPV6HCNLRIXVWOEBQPYAKQLOQYCEKZMCMWKMPCSE6F6SNLDVBM` ([Stellar Expert](https://stellar.expert/explorer/testnet/contract/CDKB4W4H2UQAVRYV5HCPTGOU4FYCKJNIHHIEUP7BR6KBHJYLZH4BZRJ5))
- Team (deployer) account: `GCTTJNFWYPHLF6YLQEBA34BPGJWP5IJFZFWGCRSW77EDRZLVFZRES2GX`

```sh
cargo test
stellar contract build
```

Testnet only. Testnet is reset from time to time, which removes the contract.

CI: `ci/github-actions-ci.yml` is the workflow, kept outside `.github/workflows/` until it can be pushed (the pushing token lacks the `workflow` scope).
