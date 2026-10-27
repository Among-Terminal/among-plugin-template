# Among plugin template

A starting point for a plugin for [Among Terminal](https://github.com/Among-Terminal/among):
one Rust managed plugin, `hello-status`, that shows a short greeting where
Among asks for status, with CI that builds, tests and packages it for Linux
amd64 and arm64, and a tag workflow that publishes an installable release.

> **Status.** Among installs plugins from a repository's latest immutable
> release with the release format this template produces. That installer is
> part of Among 0.2.0 and is still being implemented: until it ships, releases
> made from this template build and publish, but Among cannot install them yet.

## Start a plugin

1. **Use this template** on GitHub to create your repository.
2. In the repository settings, enable **immutable releases**. Among only
   installs from immutable releases, and the
   release workflow cannot check this setting for you.
3. Make sure the repository's Actions settings allow public reusable workflows: the
   workflows here call `Among-Terminal/among-plugins`.
4. Add the repository topic `among-plugins`, so a future plugin registry can
   find it.
5. Rename the plugin: the directory `plugins/hello-status/`, the manifest's
   `id`, `name`, `description` and `entrypoint`, and the `[[bin]]` name in
   `Cargo.toml`. The binary's file name must equal the last part of
   `entrypoint`.

## What a plugin is

Among starts the executable named by `entrypoint` and exchanges one JSON
object per line over stdin and stdout. `src/main.rs` implements the managed
API 1.2 handshake (`managed_hello`), answers `status` requests with fragments
and stops on `shutdown`. Change `fragments` to show your own content. If your
plugin needs more than `status`, add the features to both the manifest's
`required_features` and the `FEATURES` constant.

The full protocol, features, settings and fragment format are described in
[Among's plugin guide](https://github.com/Among-Terminal/among/blob/main/docs/plugins.md).
Until Among 0.2.0 is released, that guide on `main` still describes the
previous plugin model.

## Develop

```sh
cargo +1.98.1 fmt --all --check
cargo +1.98.1 clippy --locked --all-targets -- -D warnings
cargo +1.98.1 test --locked --all-targets
cargo +1.98.1 build --release --locked
```

Pull requests run `.github/workflows/check.yml`: the same checks plus
packaging and the release catalogue on native amd64 and arm64, without
publishing anything.

## Release

Update `version` in the manifest, commit, then push a tag:

```sh
git tag v0.1.0
git push origin v0.1.0
```

`.github/workflows/release.yml` builds both architectures and publishes an
immutable release with one package per architecture, the source delivery,
`among-plugins.json` and `SHA256SUMS`. A published immutable release cannot be
changed; a mistake ships as a new version. What the workflow does, and what to
do when a run stops, is described in
[the release guide](https://github.com/Among-Terminal/among-plugins/blob/main/docs/author-release.md).

## Licence

This template is licensed under Apache-2.0 (`LICENSE`). Replace it with your
own licence for your plugin; packaging requires a `LICENSE` file.
