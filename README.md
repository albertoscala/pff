# pff - pull files fast

`pff` is a tiny, language-agnostic dependency puller written in Rust.

It's the plumbing for this bigger project: many independent
repos that occasionally need to share a handful of files with each other (a header,
a linker script, a common Verilog package, whatever). `pff` exists to move exactly
those files from one repo to another, and nothing more.

> Status: early / work in progress. The core loop works; the ergonomics don't yet.
> Expect breaking changes.

## Why

A decade-long systems project ends up as a pile of repos written in different
languages (C, Rust, assembly, Verilog/VHDL...) that still need to hand things to
each other, a register map header the OS and the RTL both need to agree on, a
shared linker script, a common macro file.

The usual options don't fit well:

- **Git submodules** pull in an entire repo (history, unrelated files, build
  system and all) just to get one header.
- **A real package manager** (cargo, apt, pip...) is tied to one language and
  one build system, this project deliberately isn't.

`pff` picks the middle ground: declare *which repo* you need something from and
*which file* you need, and it fetches just that file into just the folder you
want. No registry, no submodule, no build-system coupling, just files, pulled
fast, straight off GitHub.

## How it works

There are two roles, and one small `toml` file for each.

**The puller**: lives in the project that *wants* things. It says who to pull
from and which repos to look at.

**The pulled**: lives in the repo that *offers* things. It says which files
are available and where the puller should put them.

When you run `pff` inside a project with a `puller.toml`:

1. It reads `puller.toml` from the current directory.
2. For every repo listed under `dependencies`, it fetches that repo's
   `pulled.toml` from
   `https://raw.githubusercontent.com/<username>/<repo>/main/pulled.toml`.
3. For every `[file, destination_dir]` pair in that `pulled.toml`, it downloads
   `https://raw.githubusercontent.com/<username>/<repo>/main/<file>` and writes
   it to `<destination_dir><file>` locally.

That's the entire tool. No caching, no locking, no dependency resolution, it
just walks the list and copies files.

## File templates

### `puller.toml`, "what I need, and from whom"

Put this in the root of the project that needs files pulled in.

```toml
[dependencies]
repos = [
    ["github-username-or-org", "repo-one-name"],
    ["github-username-or-org", "repo-two-name"]
]
```

- `dependencies.repos`: the repos to pull from, each given as a
  `["owner", "repo"]` pair. Owners can differ between entries, there's no
  requirement that they all belong to the same account.
- Each listed repo must contain a `pulled.toml` at its root, on its main
  branch. That file declares which files get pulled and where they land. 

### `pulled.toml`: "what to pull, and where to put it"

Put this in the root of the repo that's being depended on.

```toml
[setup]
files = [
    ["path/in/this/repo/to/file.h", "./include/"],
    ["src/common/regs.vh",          "./rtl/include/"]
]
```

- Each entry is `[source_file, destination_dir]`.
- `source_file` is the path to the file **inside this repo**, relative to repo
  root.
- `destination_dir` is where it lands **in the puller's project**. It's
  concatenated directly with the filename, so it currently **must end with a
  trailing `/`**, and the directory must already exist.

## Usage

```bash
git clone https://github.com/Emilia-Systems/pff
cd pff
cargo build --release
```

Then, in any project that needs dependencies pulled:

1. Add a `puller.toml` (see template above) to the project root.
2. Run the `pff` binary from that same directory.

It will walk every listed repo, read its `pulled.toml`, and pull the files in.

## Current limitations

Being upfront about where this stands today:

- Only pulls from `raw.githubusercontent.com`, GitHub only, no other hosts.
- The branch is hardcoded to `main`.
- All dependency repos in one `puller.toml` must belong to the same
  `username`/org.
- No version pinning, it always pulls whatever is on `main` right now, so a
  pull today and a pull tomorrow can differ.
- No lockfile, no caching, no dependency resolution/deduplication.
- File pulling is one file at a time, no "pull this whole directory."
- Errors currently panic (`.unwrap()` throughout) instead of failing gracefully.
- `destination_dir` needs a trailing slash, or the write path will be wrong.

None of this is fixed, it's the honest current state, and a good place to
start if you want to contribute.