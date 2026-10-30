# Zed snap: acceptance and Store hand-off

The snap is `classic`, `core26`, stable channel only, built from the `bundle-linux` tarball
(`script/snap-build`). Both `amd64` and `arm64` must pass everything below before hand-off.
Install for testing with `sudo snap install --dangerous --classic zed_<version>_<arch>.snap`.

## Manual acceptance checklist

Run on each architecture (amd64, arm64). Hosts: Ubuntu 26.04 and 24.04 only.

- [ ] **Hosts**: X11 session and Wayland session; one machine with a Vulkan GPU and one
      llvmpipe (software) setup. Zed starts and renders on each.
- [ ] **Environment pollution**: compare `env` in the integrated terminal and in a task
      against the tarball build. Any leaked `SNAP*` variable, `LD_LIBRARY_PATH`, or changed
      `PATH`/`XDG_DATA_DIRS` blocks the release. `git`, `ssh`, `cargo`, `rust-analyzer`
      must run normally from the terminal and tasks.
- [ ] **Downloads** (confirm files land in `~/.local/share/zed`):
  - [ ] install an extension that uses a Node-based language server
  - [ ] auto-download a language server (rust-analyzer or clangd)
  - [ ] download the remote server over SSH
- [ ] **Updater**: the in-app update path shows "Please use snap to update zed" and does not
      self-update.
- [ ] **Desktop**: `.desktop` entry, icon and MIME associations work; `zed .` works from a
      shell; Zed appears in GNOME and KDE launchers.
- [ ] **Review evidence**: `snapcraft lint` output (from the review artifact) with every
      warning listed and explained, plus `classic-justification.md`. Expected known false
      positives: `libvulkan1`, `libwayland-client0` unused libraries.

CI already runs `snap install --dangerous --classic` and `zed --version` in each
`bundle_snap_*` job; that does not replace this checklist.

## Future: strict

- **Blocker**: snapd has no interface for exec'ing arbitrary host binaries (terminal, tasks,
  git, ssh, language servers). This is a snapd limitation, not a Zed code problem.
- **Easy plugs** if it is ever unblocked: GPU, display (x11/wayland), audio, updater-related.
- Revisit only if snapd gains a suitable interface. A strict snap would also need a data
  migration or `personal-files`. No work is planned.

## Store submitter hand-off

Registration, credentials and publishing are manual and out of scope for CI.

1. Take the snaps from the GitHub release assets: `zed_<version>_amd64.snap` and
   `zed_<version>_arm64.snap` (uploaded by the release workflow). To rebuild without a
   release, run the manual `snap.yml` workflow (`workflow_dispatch`) and download the
   `zed-snap-x86_64` / `zed-snap-aarch64` artifacts.
2. Download the review artifacts `zed-snap-review-x86_64` and `zed-snap-review-aarch64`
   from the same workflow run: `snapcraft lint` output, `snapcraft.yaml`, and classic
   precedent (`classic-precedent.txt`).
3. Confirm the acceptance checklist above passed on both architectures.
4. Register the name `zed` (if not already), then request classic confinement review in the
   Snap Store forum, attaching `classic-justification.md` and the lint/precedent evidence.
5. After approval, `snapcraft upload --release=stable zed_<version>_<arch>.snap` for each
   architecture.
