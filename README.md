# Rust bindings for COIN-OR CBC Solver

The code is based on:
* [likr/clp-rs](https://github.com/likr/clp-rs) Here I borrowed the approach to building and binding of C++ code into a Rust package
* [KardinalAI/coin_cbc](https://github.com/KardinalAI/coin_cbc) Here I borrowed the actual library that turns bindings into normal Rust functions and tests (only the raw version of CbcModel is used, it's the one that allows to `Cbc_LoadProblem()` all at once)
* [COIN-OR Cbc/Clp/Cgl/Osi/CoinUtils](https://github.com/coin-or) Versions of the COIN-OR libs are hardcoded to be the latest releases as of November 2021 (with a patch that revives `Cbc_getRowPrice()` method in the Cbc C API)
