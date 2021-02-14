# vers

![Rust](https://github.com/ink8bit/vers/workflows/Rust/badge.svg)

`vers` updates version in JavaScript projects.

In general, `vers` does the following:

- `package.json`
- `package-lock.json` (if exists)
- creates *CHANGELOG.md* file
- creates commit using formatting: `Version bump: v0.1.0`
- creates an annotated git tag
- commits and pushes changes

## How to use

You can use it as a binary or as a library.

### Binary app

You should choose a version type: `major`, `minor`, or `patch`. And include changes:

```
vers minor --info "some changes"
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
vers::update(version, info);
```

## Changelog format

Every changelog entry has these contents:

```
## v0.1.0

### Date string in RFC 2822 format

**Releaser:** @username

**Changes:** your changes

List of commits in feature branch
```
