# vers

![Rust](https://github.com/ink8bit/vers/workflows/Rust/badge.svg)

`vers` updates version in JavaScript projects.

In general, `vers` does the following:

- updates `package.json`
- updates `package-lock.json` (if exists)
- creates _CHANGELOG.md_ file
- creates commit using formatting: `Version bump: v0.1.0`
- creates an annotated git tag
- and pushes changes

`vers` works only with _git_.

## How to use

You can use it as a binary or as a library.

### Binary app

You should choose a version type: `major`, `minor`, or `patch`. And include changes:

```console
vers minor --info "some changes"
```

#### Usage

```console
USAGE:
    vers [FLAGS] [OPTIONS] <version_type>

ARGS:
    <version_type>    [possible values: major, minor, patch]

FLAGS:
    -h, --help         Prints help information
    -n, --no-commit    Prevents committing your changes
    -V, --version      Prints version information

OPTIONS:
    -i, --info <string>    Sets info value
```

### Library

> The crate is only available via git repo for now. You can include it using `rev`, `tag` or `branch` key. Read more in [Cargo docs](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories).

You should add `vers` crate to your _Cargo.toml_ file:

```console
[dependencies]
vers = { git = "https://github.com/ink8bit/vers", branch = "master" }
```

### Public API

#### `update`

Updates `CHANGELOG.md`, `package.json`, `package-lock.json` (if exists), commits changes, creates tag and pushes changes to the remote. Returns created version value.

```rust
match vers::update("minor", "changes", false) {
    Ok(v) => println!("Updated version: {}", v),
    Err(e) => eprintln!("{}", e),
}
```

#### `save_changes`

Creates commit and tag. Returns created tag value.

```rust
match vers::save_changes("v1.2.3", "releaser", "some info") {
    Ok(v) => println!("Created tag: {}", v),
    Err(e) => eprintln!("{}", e),
}
```

#### `push_changes`

Pushes changes to the remote. Returns current git branch value.

```rust
match vers::push_changes() {
    Ok(v) => println!("Current git branch: {}", v),
    Err(e) => eprintln!("{}", e),
}
```

#### `releaser`

Returns your git user name and user email, or your GitHub handle if you set env var [VERS_GITHUB_NAME](#using-github-username).

```rust
let releaser = match vers::releaser() {
    Ok(value) => value,
    Err(e) => panic!("{}", e),
};
```

#### `current_branch_name`

Returns current `git branch` value.

```rust
let branch = match vers::current_branch_name() {
    Ok(value) => value,
    Err(e) => panic!("{}", e),
};
```

## Changelog format

Every changelog entry has these contents:

```console
## v0.1.0

### Date string in RFC 2822 format

**Released by:** username <user email>

**Changes:** your changes

List of commits in feature branch
```

### Using github username

You can use your GitHub username in:

- `Released by:` field in your _CHANGELOG.md_ file
- `Tagged by:` field in your tag
- `Released by:` field in your commit message

You need to set env variable `VERS_GITHUB_NAME`. For example:

```console
export VERS_GITHUB_NAME=username
```

> If `vers` could not get value from env var `VERS_GITHUB_NAME`, your _git_ user name and email will be used instead.
