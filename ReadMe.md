# Phase 1: As per Spec

WHat is the computational logic/ algorithm needed here?
profitable arbitrage opportunities to capitalize on.
Sometimes these new currency pairs drift in a way that creates an arbitrage path where you can convert coins through a certain sequence to return a profit in your base currency. This is referred to as an arbitrage loop.

Moore - Bellman Ford
Shortest Route
Djkistra - will not work as cannot handle none negative

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

## Thoughts on future improvements

Live Updates, multi threaded
Integration Tests - provide configurable API call address
Configurable
