# Git Stratum

<p style="text-align:center;">
    <b>Stratum</b>: A single layer of something - <a 
        href="https://dictionary.cambridge.org/dictionary/english/stratum">
        Cambridge Dictionary
    </a> 
</p>

A library that lets you anlayse your repository one **strata** at a time, leveraging a higher level API than [git2-rs](https://github.com/rust-lang/git2-rs) analysing your repository is simple!

```bash
cargo add git-stratum
```

## Usage

*This is currently unstable and any and all versions of the API up untiil version 1 should be considered unstable. The below usage is the current vision for the project.*

```rust
use stratum::Repository;

let repo = Repository::from_str("path/to/repo");
for commit in repo.iter_commits() {
    ...
}
```

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
