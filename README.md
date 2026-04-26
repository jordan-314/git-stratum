# Git Stratum

**Stratum**: A single layer of something - [Cambridge Dictionary]("https://dictionary.cambridge.org/dictionary/english/stratum").

A library that lets you anlayse your repository one **strata** at a time, leveraging a higher level API than [git2-rs](https://github.com/rust-lang/git2-rs) analysing your repository is simple!

## Quick Start

First add `stratum` to your dependencies.

```bash
cargo add git-stratum
```

Then inside of your main function:

```rust
use stratum::open_repository;

let repo = open_repository("path/to/repo").unwrap();
for commit in repo.iter_commits().unwrap() {
    let commit = commit.unwrap();
    ...
}
```

*Note that the API is liable to change up until version 1.0.0.*

For more detail on the API, see the [docs]().

## Testing

**credit:** The `/test-repos.zip` file was originally created by the maintainer of [PyDriller](https://github.com/ishepard/pydriller/tree/master), which is the core inspiration for this project. At the time of writing Pydriller is under the Apache2.0 license.

To write new tests:
- Manually unzip the `test-repos.zip` choose a repository to use in testing.
    - If one is not appropriate, create a new one, **be sure to upload a new zip file containing your new repository**.
- To use your chosen repository, use a code block similar to the following:

```rust
mod common;
use common::test_data_dir;

fn small_repo() -> PathBuf {
    test_data_dir().join("small_repo")
}
```

- The `test-repo` should only be unzipped once per test module, so if your testing fits within an existing test module, it will save some time to put the test in an existing module.
