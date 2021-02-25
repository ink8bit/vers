# vers

![Rust](https://github.com/ink8bit/vers/workflows/Rust/badge.svg)

`vers` updates version in JavaScript projects.

In general, `vers` does the following:

- `package.json`
- `package-lock.json` (if exists)
- creates *CHANGELOG.md* file
- creates commit using formatting: `Version bump: v0.1.0`
- creates an annotated git tag
- and pushes changes

## How to use

You can use it as a binary or as a library.

### Binary app

You should choose a version type: `major`, `minor`, or `patch`. And include changes:

```
vers minor --info "some changes"
```

#### Usage

```
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

You should add `vers` crate to your *Cargo.toml* file:

```
[dependencies]
vers = { git = "https://github.com/ink8bit/vers", branch = "master" }
```

And use this crate in the following way:

```rust
match vers::update("minor", "changes", false) {
    Ok(v) => println!("Updated version: {}", v),
    Err(e) => eprintln!("{}", e),
}
```

There are two more public functions you can use:

- `save_changes` - to commit and tag your changes with a provided version string
- `push_changes` - to upload your changes to the remote

Be sure to check for errors while using these two functions:

```rust
if let Err(e) = vers::save_changes(&v) {
    panic!(e);
}

if let Err(e) = vers::push_changes() {
    eprintln!("{}", e);
}
```

## Changelog format

Every changelog entry has these contents:

```
## v0.1.0

### Date string in RFC 2822 format

**Released by:** username <user email>

**Changes:** your changes

List of commits in feature branch
```

### Using github username

You can use your GitHub username in **Released by:** field of your *CHANGELOG* file instead of *git* user name and email.
You need to set env variable `VERS_GITHUB_NAME`. For example:

```
export VERS_GITHUB_NAME=username
```
