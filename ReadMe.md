# Pre-requisites

1. Cargo and rust toolchain.
2. Mermaid extension for your chosen IDE.

# Requirements

## Functional

1. Application must be able to make an API call to https://api.swissborg.io/v1/challenge/rates to access latest rates (updated every 10 seconds).
2. Discovers arbitrage opportunities to provide user with most profitable route.

## Non Functional (what is the problem)

1. Reusable Algorithm for scalability - allow for multi threaded and live updates in the future.
2. Testable - unit tests provided for some functionality.

Out of scope:

1. Receiving live updates - You only have to pull the data once per run of your algorithm - this is assumed per program run.
2. Integration Tests of entire service? Maybe run some in the test folder?

Assumptions:

1. The rates for conversions between currencies/ cryptos can be removed i.e. BORG-BORG - this way we do not build a map with replicated nodes.
2. No costs per trade.

# Build

`cargo build`

# Test

`cargo test`

# Run

`cargo run`
