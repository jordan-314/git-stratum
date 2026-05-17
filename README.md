<h1 align="center">
    Git Stratum
</h1>

Git Stratum, or more concisely, `stratun` is a git repository mining library written in rust. It's predominantly based on [Pydriller](https://github.com/ishepard/pydriller/) and intends to follow a similar feature root, though dividing some of the variety into individual crates.

## Workspace Structure

If you're not familiar with cargo workspaces, the see [the rust book](https://doc.rust-lang.org/stable/book/ch14-03-cargo-workspaces.html#cargo-workspaces). The repository is subdivided into the scrates which are named after their intended import name. The name of each crate on `crates.io` (if published) can be found in the individual `Cargo.toml` files.

The repository is structured as a typical `worksapce` but here is a mapping of what the individual crates are for clarity:

- `stratum`: 
    - The core library, everything depends on this unless otherwise stated.
    - Published: Yes
- `ffi`:
    - The `stratum` FFI, currently this is a placeholder for the future work.
    - Published: Yes (intention)
- `stratum-utils`:
    - Utility functions that are common to the `stratum` package, currently limited to testing utilities, may be expanded in the future.
    - **Does not depend on** `stratum`
    - Published: No

### Future Crates

This is a shortlist of future work that is intended and will exist in its own crate.

- `stratum-dmm`: See `Pydriller` DMM functionality for more insight as to what this will be.
- `stratum-metrics`: See `pydriller.process_metrics` for more insight.

## Project Scope

This project looks to create a repository mining framework written in Rust. Obviously a lot of research is not done in rust, therefore, the project will look to build an FFI layer. It has not been decided whether this will be a C-FFI or a PyO3 specific FFI. IF it is the former, then this repository will **not** wrap the C-FFI here, some language wrappers, namely Python, will be managed alongside this repository. However, if you want access in another language that does not exist, then the earnest to create the wrapper around the FFI falls to you.
