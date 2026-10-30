# Classic confinement request for `zed`

## Summary

Zed is a code editor and IDE. IDEs are a pre-approved category for classic
confinement in the Snap Store review process, and Zed has the same need as the
other IDE snaps: it must run and interoperate with the user's own tools on the
host.

## Justification

1. **Category.** Zed is an integrated development environment (a
   GPU-accelerated code editor with a built-in terminal, language-server
   client, debugger, and task runner). IDEs and code editors are listed among
   the known pre-approved classic categories.
2. **It runs user-chosen host tools.** Zed's core workflow is launching
   programs the user picked, from wherever they are installed on the host:
   - language servers, formatters, linters, and compilers (`rust-analyzer`,
     `gopls`, `clangd`, `node`, `python`, `cargo`, `go`, ...) from `PATH`,
     toolchain managers (`rustup`, `nvm`, `asdf`, `direnv`, `nix`), and
     per-project virtual environments;
   - the user's login shell in the integrated terminal, and `git` and `ssh`
     (with the user's agent, keys, and config);
   - project files anywhere on the filesystem, not only under `$HOME`.
3. **Strict confinement cannot express this.** Confinement would hide the host
   toolchains, `PATH`, shell configuration, and project directories, and would
   give child processes a different environment from the one the user set up.
   Interfaces such as `home` or `system-files` do not cover arbitrary
   toolchain locations, and a sandboxed terminal would not be a usable
   terminal. Zed also keeps its normal data paths (`~/.config/zed`,
   `~/.local/share/zed`) rather than `~/snap/zed`.
4. **Scope of what the snap does.** The snap contains only the Zed binary
   built by the upstream release pipeline plus a small set of stage-packages
   (`libasound2t64`, `libvulkan1`, `libwayland-client0`, `libxcb1`,
   `libxkbcommon-x11-0`, `libxkbcommon0`). It has no daemons or services, and
   it disables the self-updater (`ZED_UPDATE_EXPLANATION`) so that updates come
   only through the Store.

## Precedent: comparable editors and IDEs are classic snaps

Confinement as reported by the Snap Store API
(`https://api.snapcraft.io/v2/snaps/info/<name>?fields=confinement`,
stable channel), checked 2026-10-30:

| Snap                        | Product              | Confinement |
| --------------------------- | -------------------- | ----------- |
| `code`                      | Visual Studio Code   | classic     |
| `sublime-text`              | Sublime Text         | classic     |
| `intellij-idea-community`   | JetBrains IntelliJ IDEA Community | classic |
| `pycharm-community`         | JetBrains PyCharm Community       | classic |
| `nvim`                      | Neovim               | classic     |

Reproduce with:

```sh
for s in code sublime-text intellij-idea-community pycharm-community nvim; do
  curl -s -H 'Snap-Device-Series: 16' \
    "https://api.snapcraft.io/v2/snaps/info/$s?fields=confinement" |
    jq -r '"\(.name): \(.["channel-map"][0].confinement)"'
done
```

## `snapcraft lint` results

The manual `snap` workflow attaches the real `snapcraft lint` output for each
architecture (`snapcraft-lint-<arch>.txt`) to its artifacts. The warnings
expected there, and why each is acceptable:

- **Classic linter** (`classic`): reports that the snap is classic and
  checks that the ELF binaries find their libraries. Zed's binary is patched
  by `enable-patchelf`, so the classic linter has nothing to fix. The
  classic confinement itself is justified above.
- **Library linter, unused library `libvulkan1.so.1`** (false positive): Zed
  loads Vulkan at runtime with `dlopen` (through `wgpu`/`ash`), so the library
  is not in the ELF `NEEDED` list and the linter sees it as unused. Without it
  Zed exits with `Missing Vulkan entry points: LibraryLoadFailure`.
- **Library linter, unused library `libwayland-client.so.0`** (false
  positive): Zed also loads this at runtime with `dlopen`. Without it Zed
  exits with `NoWaylandLib` when run under Wayland.

These are warnings only; they do not fail the build and are not blocking
review. The library linter is not disabled in `snapcraft.yaml`, so reviewers see
exactly what it reports.
