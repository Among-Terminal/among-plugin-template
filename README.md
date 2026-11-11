# Among plugin template

A Rust template for [Among Terminal](https://among-terminal.github.io/) plugins.
The `hello-status` executable answers status requests with a greeting. It uses
managed plugin API 1.2, implemented in Among 0.2.0.

This repository contains the plugin author guide and API reference. For terminal
usage and plugin installation, use the [Among website](https://among-terminal.github.io/plugins/).
For work on Among itself, see [Contributing](https://github.com/Among-Terminal/among/blob/main/CONTRIBUTING.md).

## Quick start

### Create and build

1. Choose "Use this template" on GitHub and clone your new repository.
2. Start with `hello-status`. When renaming it, update these locations:

   - Rename `plugins/hello-status/` and update the manifest's `id`, `name`,
     `description` and `entrypoint`.
   - Update both `[package].name` and `[[bin]].name` in `Cargo.toml`. Refresh
     the corresponding package entry in `Cargo.lock` before running the
     `--locked` checks.
   - In `src/main.rs`, update the manifest test's
     `include_str!("../plugins/hello-status/among-plugin.json")` path and
     `assert_eq!(manifest["entrypoint"], "bin/hello-status")` expectation.
     The binary filename must match the last component of `entrypoint`.
   - In the same file, update the manifest path in the module comment. Keep
     both `github:1/hello-status` handshake fixture identities consistent if
     you rename them; the test checks that the reply echoes the supplied identity.
   - Update the executable name and build-output path in this README, and
     the manifest link's text and target in `docs/guide.md`.
   - Adapt the package tree, build and manifest paths, archive filename and
     `members` example in `docs/releasing.md`. Its `hello-status/v1.2.3`
     example illustrates an unsupported tag format; keep release tags in
     the documented `v*` form.

   The local files in `.github/workflows/` contain no hardcoded `hello-status`
   references to rename. After renaming, run the [template checks](#check-and-release).
3. Build from the repository root:

```sh
cargo build --release --locked
```

The default build produces `target/release/hello-status`. Edit `fragments` in
[`src/main.rs`](src/main.rs) to change the greeting. The
[author guide](docs/guide.md) explains how to add capabilities and handle scopes.

### Try it in Among

Prepare a package containing the build output and manifest, with an adjacent
schema-2 `among-plugins.json`. Follow [local package preparation](docs/releasing.md#prepare-a-local-package)
for the layout and metadata. Copying the raw executable into a plugin directory
does not register it with Among.

Quit Among, then start:

```sh
among --plugin-development
```

Use the local catalogue flow in [development mode](docs/guide.md#development-mode)
to test the package. The template accepts the local identity supplied by Among.
After enabling it with its requested access, expect `hello` in the application
status areas. It does not request terminal context.

### Check and release

The `cargo +1.98.1` commands require rustup's Cargo proxy. Install rustup if it
is missing, then install the toolchain with the formatter and linter:

```sh
rustup toolchain install 1.98.1 --profile minimal --component rustfmt --component clippy
```

Run the template checks:

```sh
cargo +1.98.1 fmt --all --check
cargo +1.98.1 clippy --locked --all-targets -- -D warnings
cargo +1.98.1 test --locked --all-targets
```

The [Check workflow](.github/workflows/check.yml) calls the pinned reusable
workflow with `publish: false`. The [Release workflow](.github/workflows/release.yml)
calls it on stable `v*` tags. Review [packaging and releases](docs/releasing.md)
before publishing, including version selection and immutable-release settings.

## Documentation

| Page | Contents |
| --- | --- |
| [Author guide](docs/guide.md) | Process model, access, settings, confinement, development mode and testing. |
| [API reference](docs/api.md) | Messages, manifest fields, grants, scopes, limits and JSON exchanges. |
| [Packaging and releases](docs/releasing.md) | Package layout, catalogue, verification, versions and publishing. |

## Licence

This template uses [Apache-2.0](LICENSE). Choose the licence for your plugin and
include the required licensing and attribution material in its package.
