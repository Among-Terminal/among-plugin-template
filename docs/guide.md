# Writing a plugin

Start with the [template quick start](../README.md#quick-start). This guide
covers author decisions for Among 0.2.0. Exact fields and bounds live in the
[API reference](api.md); package and release rules live in
[packaging and releases](releasing.md).

## Process model

Among launches your executable as a separate process in its installed package
directory. It draws the interface from the data your plugin returns. Plugins do
not load code into Among or create GTK widgets.

Read newline-delimited JSON from stdin and write replies to stdout. Complete the
handshake before handling other work, retain its session and generation, and
flush each reply. Keep ordinary logs off stdout. See
[transport and lifecycle](api.md#transport-and-lifecycle) for framing, shutdown
and the message-flow diagram.

The template's `State` handles admission and status requests.
`fragments` builds the visible content. It currently returns the same greeting
for every scope it receives. Change that function to select content by
`scope.slot`. Handle EOF and shutdown without leaving background work running.

The API cannot type into a terminal, read scrollback, open or close panes, change
the layout, or execute a command in the user's shell. Terminal context exposes
only the granted fields.

## Manifest and access

The template manifest is
[`plugins/hello-status/among-plugin.json`](../plugins/hello-status/among-plugin.json).
Its `entrypoint` names the executable inside the package. Keep the package's
identity, version and manifest bytes consistent with the
[catalogue](releasing.md#catalogue).
The [manifest reference](api.md#manifest) defines every field and its validation.

Declare essential capabilities in `required_features` and useful additions in
`optional_features`. Required access must be accepted before the plugin can run.
Optional access starts unchecked at the first review. Implement the plugin so
that refusing an optional feature still leaves its essential function usable.

For example, a plugin can show application status without knowing terminal
directories. If directory information is optional, handle its absence and
request the terminal-context dependency too. See the
[feature table](api.md#feature-grants) for names and dependencies, and
[terminal context](api.md#terminal-context-and-enumeration) for the distinction
between an omitted field, an unknown value and unavailable text.

Keep the manifest and handshake feature selection consistent. When adding a
required capability to this template, update both `required_features` and the
`FEATURES` constant in `src/main.rs`. An optional capability needs explicit
selection from the host's offer; adding it to the constant would make this
template require it. Access changes require stopping the running plugin first.
Saving an access review does not restart the plugin or enable a delivery channel.

## Scopes and fragments

A status request names a scope: a slot and the application, workspace or pane
that owns it. Use the host-issued target reference, not a sidebar position.
Answer the request ID you received. Among associates the reply with that
request's scope.

Use [targets and scopes](api.md#targets-and-scopes) to choose which requests your
plugin handles. The [fragment reference](api.md#status-and-fragments) defines
text, icons, meters, ordering and refresh behavior, including the sidebar
summary exception. Supply readable text with a meter so its value has a label.

When you need more context, follow the
[enumeration exchange](api.md#terminal-context-and-enumeration). Context is
limited to the requested target. A directory string does not grant filesystem
access to that directory. Directory changes depend on the terminal reporting
them, including through OSC 7; do not infer them from focus changes.

## Notification roles

Choose the role your plugin needs:

- A producer submits an event for the user's notification policies to handle.
- A UI contributor supplies bounded text and buttons for Among to render.
- A delivery provider declares channels and handles jobs for channels the user
  has enabled.

An event does not prove that a sound played or a desktop notification arrived.
A button sends an action message to the plugin, not a shell command. Delivery
access still obeys the sandbox. The [notification reference](api.md#notifications)
defines each exchange, expiry, cancellation and outcomes; the
[grant rules](api.md#feature-grants) define which roles can coexist.

## Settings and persistent data

Declare a settings schema in the manifest and negotiate the `settings` feature.
Among builds the settings page and validates values before sending them. Handle
each configuration revision and report whether it was applied, rejected or
requires a restart. Saving settings is separate from applying them and does not
enable a disabled plugin. See [configuration](api.md#configuration) and the
[settings schema](api.md#settings-schema).

Use `AMONG_PLUGIN_DATA_DIR` for files your plugin owns. Do not derive this path
from `HOME` or assume it is the package directory. Treat the package as read-only.
The private data directory permits reads and writes, but execution from it is
blocked under Landlock confinement. Among creates its leaf directory with mode
`0700` and refuses startup if an existing leaf has a different mode. Uninstalling
a plugin retains its settings and private data.

## Environment

Among clears the inherited environment and sets:

| Variable | Value |
| --- | --- |
| `PATH` | `/usr/local/bin:/usr/bin:/bin` |
| `LANG` | `C.UTF-8` |
| `AMONG_PLUGIN_DATA_DIR` | The plugin's private data directory. |
| `TZ` | Present only if supplied to Among; its bytes are preserved. |

An absent `TZ` and an empty `TZ` stay distinct. Among does not trim or interpret
the value. More than 1024 bytes or any ASCII control byte makes the plugin
unavailable before spawn. Timezone file access remains subject to confinement.
`HOME` and the parent's `XDG_*` variables are not inherited.

## Confinement

Among applies the available Landlock and seccomp protection before executing the
plugin. The child must report sufficient protection before startup can continue.
Children inherit the sandbox.

With full protection, the process can read its package and the system paths
`/usr`, `/lib`, `/lib64`, `/etc/ld.so.cache` and `/etc/localtime` where present.
It can execute programs from the package and system directories. The
`system.metrics` grant adds the read-only files listed in the
[feature reference](api.md#feature-grants).

Under Landlock, `/proc/self` is unreadable even with `system.metrics`.
Your language runtime must tolerate denied reads from `/proc/self/...` during
startup. Filesystem usage queries through `statvfs` need no feature grant.

Seccomp blocks network and local sockets, debugging and memory access to other
processes, signal sending, shared IPC and selected kernel operations. It also
blocks changes to file metadata, including permissions, ownership, timestamps
and extended attributes. Plan data writes without relying on `chmod`, `chown`
or timestamp changes at runtime.

Denied operations return `EPERM` or `EACCES`. A system call using a foreign ABI
kills the process instead. On x86-64, x32 system calls also kill the process.

| Available protection | Author-visible behavior |
| --- | --- |
| Landlock ABI 3 or newer plus seccomp | Full protection. |
| Landlock ABI 1 or 2 plus seccomp | User consent required; truncation of writable files outside allowed paths cannot be prevented. |
| Seccomp without usable Landlock | User consent required; filesystem access follows the user's normal permissions. |
| No usable seccomp | Startup is refused. |

Do not depend on the extra access a reduced-protection environment happens to
allow. The sandbox sets no CPU or memory quotas and cannot protect against
kernel vulnerabilities.

## Limits and failure handling

Bound output and work queues in the plugin. The reference lists
[transport limits](api.md#transport-and-lifecycle),
[status bounds](api.md#status-and-fragments),
[notification limits](api.md#notifications) and
[failure handling](api.md#errors-and-failure-handling).

An invalid reply cannot complete a pending request. Prefer an empty status result
or a truthful unavailable label when a measurement fails. Exit on an
unrecoverable failure. On a new process generation, discard old request IDs and
complete admission again. Do not assume a restart will replay delivery work.

## Versioning

Use the [version negotiation rules](api.md#version-negotiation) for API and
settings compatibility. Use [release versioning](releasing.md#versions) for
package versions and update ordering. The template targets the API offered by
Among 0.2.0; check its handshake before targeting a newer host API.

## Development mode

Quit Among before starting `among --plugin-development`. The flag applies to
the first instance only and is not saved. Passing it to a running instance exits
with status 2.

Prepare the [local package and catalogue](releasing.md#prepare-a-local-package).
Preferences then offers "Install from a local catalogue" and a
"Stable / Pre-release" selector, initially on Pre-release. Choose
`among-plugins.json` with its package files alongside it. Opening the file reads
metadata only; "Review…" begins the installation flow. For the normal access
review and enable steps, follow the
[end-user plugin instructions](https://among-terminal.github.io/plugins/).

The catalogue, packages and containing directory must belong to you. They must
not be writable by others or by a group other than your primary group. Symbolic
links are refused. Changing catalogue bytes, package size or hashes after review
refuses installation.

Local identity is `development/<plugin-id>`; repository identity is
`github:<numeric-repository-id>/<plugin-id>`. Echo the identity supplied during
admission. Local packages have no verified repository, publisher or release
provenance.

A published binary that checks a compiled
`github:<numeric-repository-id>/<plugin-id>` identity rejects the local identity.
If it exits and closes stdout during that check, Among reports
"plugin stdout closed". Install that binary from its release or prerelease, or
build a version that accepts `development/<plugin-id>` for local testing.

Replacing a local installation or replacing an installed package with a
prerelease requires confirmation. It starts disabled without the previous
access grants, and the previous generation remains available. Development and
prerelease installations remain listed in normal runs but start only in
development mode. "Technical details" offers "Recover previous…" in that mode;
recovery still checks compatibility and access.

Prerelease lookup examines at most three pages of 100 releases and selects the
non-draft prerelease with the latest creation time among those observed. It does
not save the prerelease catalogue. Package and release verification still apply.
For candidate publishing and promotion, see
[release candidates](releasing.md#release-candidates).

## Testing

Run the [template checks](../README.md#check-and-release). Its unit tests cover
the handshake bindings, admission before status, stale generations, missing
required features, shutdown and agreement with the manifest.

Use the [complete API exchanges](api.md#version-negotiation) as inputs when
testing your message loop. Compare parsed JSON fields, not object-key order.
Test absent optional grants, empty context, stale replies and invalid input.
For settings, test each outcome and a schema-version mismatch. For notifications,
test expired contributions, stale actions, cancellation and an uncertain
delivery result when your plugin implements those features.

Test the packaged plugin in Among as well. Check every scope it uses, restart it,
change settings, and test with optional access refused. Verify writes go to the
announced data directory and the plugin works under full protection. A successful
Cargo build or a direct stdin/stdout test does not exercise installation,
admission, rendering or confinement.

## Implementation sources

- [Managed runtime](https://github.com/Among-Terminal/among/blob/v0.2.0/src/plugin/host/managed_runtime.rs)
  and [context](https://github.com/Among-Terminal/among/blob/v0.2.0/src/plugin/managed.rs).
- [Environment](https://github.com/Among-Terminal/among/blob/v0.2.0/src/plugin/host/environment.rs),
  [sandbox policy](https://github.com/Among-Terminal/among/blob/v0.2.0/src/plugin/sandbox/policy.rs)
  and [protection levels](https://github.com/Among-Terminal/among/blob/v0.2.0/src/plugin/sandbox/level.rs).
- [Landlock rules](https://github.com/Among-Terminal/among/blob/v0.2.0/src/plugin/sandbox/launch.rs)
  and [seccomp filter](https://github.com/Among-Terminal/among/blob/v0.2.0/src/plugin/sandbox/bpf.rs).
- [Local installation](https://github.com/Among-Terminal/among/blob/v0.2.0/src/plugin/install/local.rs)
  and [plugin manager](https://github.com/Among-Terminal/among/blob/v0.2.0/src/prefs/plugin_manager.rs),
  with [process transport errors](https://github.com/Among-Terminal/among/blob/v0.2.0/src/plugin/process.rs).
