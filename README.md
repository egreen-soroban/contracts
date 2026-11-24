# eGreen · Soroban contracts

Soroban smart contracts for eGreen's progressive pre-financing of carbon projects on Stellar, built under the Stellar Community Fund (SCF #46, Build Award, Open Track).

**Scope of this repository:** only the Soroban contracts, their tests, deployment scripts and documentation. eGreen's application layer (web, backend, KYC/KYB service, ERP integration) is private and not part of this repository.

## Contracts (tranche 1–2)

| Contract | Role | Lead |
|---|---|---|
| `identity_registry` + `compliance` | Verified holders (T-REX); compliance checks on every transfer | Vottun / eGreen (compliance) |
| `rights_receipt` | SEP-57 rights receipts, one instance per funding round | Vottun |
| `round_manager` | Projects, rounds (price, cap, dates), queries and events | eGreen (configuration, queries, events) / Vottun |
| `round_vault` | SEP-56 multi-round escrow, tranche release, pro rata refunds | Vottun |
| `milestone_oracle` | Registered signers, signature verification, thresholds, evidence hash | Vottun |
| `retirement` | Conversion of receipts into credits, burn, retirement record | Vottun / eGreen (serials and metadata) |
| `funds_ledger` | Public use-of-funds records | eGreen |

The repository currently contains a placeholder contract (`contracts/scaffold`) that keeps CI running until the tranche 1 contracts are added.

## Standards and dependencies

| Item | Version checked against | Status |
|---|---|---|
| SEP-56 Tokenized Vault | 0.1.2 | Not ratified (see form) |
| SEP-57 T-REX | 0.4.0 | Draft |
| SEP-45 Web Auth for Contract Accounts | 0.1.1 | Draft |
| OpenZeppelin Stellar Contracts | 0.8.0 | Pinned in `Cargo.toml` |
| soroban-sdk | 28.0.0 | — |

The milestone extension of the vault is our own addition and is not part of SEP-56. If a SEP changes before mainnet, the change is absorbed behind stable internal interfaces and this table is updated.

## Build and test

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test --all
stellar contract build   # requires stellar-cli v25.2.0+
```

## Contributing

- Single repository for both teams; CI is mandatory on every pull request.
- Every change under `/contracts` requires approval from Vottun, enforced through `CODEOWNERS`.
- Acceptance criteria per module type are in the pull request template (coverage ≥ 90%, 100% on fund paths, Vottun security review for funds and permissions modules).

## License

MIT (see `LICENSE`).
