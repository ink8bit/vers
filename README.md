# vers

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

You should choose a version type: `major`, `minor`, or `patch`. And include changes

```
vers minor --info "some changes"
```

## Changelog format

Every changelog entry will be look like:

```md
## v0.1.0

### Date string in RFC 2822 format

**Releaser:** @username

**Changes:** your changes
```
