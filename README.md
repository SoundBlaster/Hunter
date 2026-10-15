# Hunter

Reusable filesystem acquisition for in-process search engines. `hunter-fs` is an
initial Unix scoped-metadata primitive extracted from BuildHunter, not a full scanner.
It has no BuildHunter/Rifle dependency or artifact vocabulary.

```rust
# #[cfg(unix)] {
use hunter_fs::{ScopedMetadata, Location};
use std::{path::Path, ffi::OsStr};
let root = Path::new("/selected/tree");
let scope = ScopedMetadata::open(root)?;
let observation = scope.inspect_regular_file(root, Location::Own, OsStr::new("config"));
# }
# Ok::<(), std::io::Error>(())
```

The root descriptor remains anchored after path replacement. Intermediate opens
and final metadata observations do not follow symlinks. Regular files return true;
missing entries, directories and links return false. Access/scope/cloud/I/O failures
stay distinct unavailable outcomes. `metadata_attempted` counts the final metadata
syscall only, not descriptor opens or `fstat`. Invalid names return without I/O.
Paths/names retain OS bytes. Names must be one nonempty component, without `/`, NUL,
`.` or `..`. Own/Parent select the candidate directory or its parent; root Parent
cannot escape the selected scope. Candidates containing parent components are rejected
before deriving Parent, with no final metadata attempt.

The host must hold effective permissions and set a non-materializing I/O policy on
the calling thread before opening/using this object on macOS. Dataless detection
alone does not establish a no-download guarantee. Calls are synchronous; transient
file changes are not an atomic snapshot. A selected regular-file root has no directory
handle and observations return unavailable. `Drop` releases owned descriptors.

Windows currently compiles without this Unix API. No traversal, size accounting,
worker pool, cancellation, indexing, full memory budget or performance gain is claimed.

Tests preserve the prior marker race cases and cover public name validation,
regular/directory/link distinctions and selected-root parent scope. Linux CI covers
non-UTF-8 names; APFS rejects creation of that fixture. Cloud evidence is synthetic,
not a live provider test.

Run `cargo test --locked`, `cargo test --locked --release`, and
`cargo clippy --locked --all-targets -- -D warnings`. CI runs these small crate checks
on Linux/macOS/Windows; no app builds or whole-disk benchmarks.

Derived from BuildHunter `crates/buildhunter-plugin-scanner/src/scope.rs` at
`96780d7f9a9a9e5a6a19a53e2eea2f4f4ff1a2aa`, under the included MIT license.
[Draft boundary](specs/scoped-metadata.spec.yaml), [map](specs/hypercode/current.hc).

## Portable primitives

`crates/hunter-fs-core` now owns portable one-directory enumeration and metadata
byte calculation, without libc or any external dependencies. See its README and
`specs/portable.spec.yaml`. This is an additional partial extraction; recursion,
worker orchestration and complete scan lifetime remain host-owned. Workspace CI
checks both packages. Prior scoped-metadata test records remain historical.

The std-only core also contains generic WorkQueue and scoped worker launch/shutdown.
It supports recursive task expansion while selection, recursion policy, setup/QoS
and report lifecycle remain consumer-owned; see the core README and worker spec.

DirectoryTask/Listing/Result and measurement planning now provide typed opaque
job/result correlation and aligned partial measurements. Host keeps recursive
selection, backends and protocol publication; see the core README and task spec.

ChildTraversal now emits host-selected recursive directory requests or indexed leaf
observations from an owned listing. Host keeps guard/selection effects, assigns
IDs, schedules requests and owns recursive accounting/status. See the core README
and specs/child-traversal.spec.yaml; this is not a complete recursive scanner.

DirectoryTaskLedger now centralizes serial task IDs, opaque listing/deferred state
and expected-response bookkeeping. Caller still owns the result protocol and
actual worker/queue lifecycle; see specs/task-ledger.spec.yaml and the core README.

SubtreeProgress now supplies saturating byte totals, pending-directory/root progress
and one-shot partial completion values. Host routes contributions and owns dependent
completion order; see specs/subtree-progress.spec.yaml and the core README.
