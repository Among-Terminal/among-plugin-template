# Versions and releases

This describes how a plugin repository made from this template versions and releases
its plugins. It follows the cycle of
[`Among-Terminal/among-plugins`](https://github.com/Among-Terminal/among-plugins/blob/main/docs/releasing.md),
where the official plugins live.

## Versions

Every plugin has its own semantic version in `plugins/<id>/among-plugin.json`. Raise it
only when that plugin changes, and leave it alone when it does not. A plugin changes when
its packaged content changes: the executable, the manifest apart from `version`, or the
attribution and dependency files in the package. Compatibility with Among is not the
plugin version: it is the `api` major and minor the manifest declares.

Use the usual meaning of semantic versions: a fix raises the patch number, a new
capability the minor number, and a change that breaks users of the plugin (for example
removed settings) the major number.

The Cargo package version in `Cargo.toml` is not what Among reads. Keep it equal to the
manifest version if you want one number in `cargo` output, but the manifest decides.

## Releases are snapshots

A release is one immutable GitHub release of the repository with one `v*` tag. Its
`among-plugins.json` lists exactly one version of every plugin in the repository. With
several plugins in one repository, plugins that did not change keep their version in the
next release and plugins that did carry their raised version.

The tag is a label of the snapshot, not a plugin version. A tag such as
`hello-status/v1.2.3` does not work: Among reads the repository's latest release, whose
catalogue describes only that release, and the release workflow accepts only tags of the
form `v[A-Za-z0-9._-]`.

## Making a release

A release goes through a release pull request, so that the versions are raised once, for
every plugin that changed, and checked before anything is tagged:

1. Open a pull request titled `chore(release): v<label>` (for a candidate,
   `chore(release): v<label>-rc.N`) that raises the `version` in `plugins/<id>/among-plugin.json`
   of every plugin that changed since the previous release, and changes nothing else. The
   `Check` workflow builds and packages exactly as a release would and publishes nothing; wait
   for it, and for `pr-title`, to pass.
2. Merge it (squash). Do not change the plugins in the same pull request.
3. Create the tag `v<label>` on the merge commit and push it to `origin`. A stable tag, one
   without a hyphen, starts the `Release` workflow, which publishes the immutable release.
   Any tag with a hyphen, a candidate included, starts nothing; see
   [Release candidates](#release-candidates).

The official repository automates steps 1 and 3 with `among-tools plugin-release-pr`, which
works out the label and the versions from the Conventional Commit titles, and a workflow
that creates the tag after it verified the merge commit. This template does not carry those
tools; follow the same steps by hand, or copy them from the official repository. See its
[release guide](https://github.com/Among-Terminal/among-plugins/blob/main/docs/releasing.md#the-release-pull-request).

The release gets a description written for people (what the release is, a table of the
plugins with version, plugin API and package links, how to install and verify, the source
commit and runs). The machine line that ties the release to its run stays at the end of the
description inside an HTML comment, which GitHub does not show; do not edit it.

The official plugin repository enforces this with `among-tools changed-plugins`, which
compares each plugin's packaged content with the previous release on every pull request.
This template does not run that check, because the tool lives in the official repository;
compare by hand, or copy the check from there if you keep several plugins in one
repository.

## Release candidates

A candidate is a prerelease with a tag `v<label>-rc.N`. It never becomes the repository's
latest release, so ordinary installation does not see it, and Among's development mode
(`--plugin-development`, "Use latest prerelease" for the repository) installs it for
testing.

The tag workflow of this template publishes stable releases only: it does not pass the
`prerelease` input, and its trigger excludes tags that contain a hyphen (`!v*-*`), so
pushing `v1.0.0-rc.1` starts no release at all (a candidate published through it would
become the latest release). To publish a candidate, call the reusable workflow from a workflow
that you start by hand, with `publish`, `prerelease: true` and the three prepared inputs
(run, artifact and checksum of a preparation you have inspected), as described in the
[release guide](https://github.com/Among-Terminal/among-plugins/blob/main/docs/author-release.md#publishing-approved-preparation-bytes).
The stable release is then the tag `v<label>` on the same commit with the same bytes. If
you fix something after a candidate, raise that plugin's version and make another
candidate.

Tagging and publishing are your decisions: in this template no pull request creates a tag
or a release.

## Pull requests

Pull requests merge by squash and the pull request title becomes the commit subject. The
`pr-title` check requires a Conventional Commit title: `feat`, `fix`, `docs`, `test`,
`ci`, `build`, `perf`, `refactor` or `chore`, an optional `(scope)`, and `!` before the
colon for a breaking change.
