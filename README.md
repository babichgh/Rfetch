# Rfetch

A fastfetch clone written in Rust.

```
             .',;::::;,'.                OS     : Fedora Linux 44 (KDE Plasma Desktop Edition)
         .';:cccccccccccc:;,.            Host   : babich@fedora
      .;cccccccccccccccccccccc;.         Kernel : Linux 7.2.5-200.fc44.x86_64
    .:cccccccccccccccccccccccccc:.       Uptime : 5h 23m
  .;ccccccccccccc;.:dddl:.;ccccccc;.     CPU    : Intel(R) Xeon(R) CPU E5-2680 v3 (12) @ 2.97 GHz : 1%
 .:ccccccccccccc;OWMKOOXMWd;ccccccc:.    GPU    : NVIDIA Corporation TU116
.:ccccccccccccc;KMMc;cc;xMMc;ccccccc:.   RAM    : 5.8 GiB / 15.4 GiB (37%)
,cccccccccccccc;MMM.;cc;;WW:;cccccccc,   Disk   : /: 289.2 GiB / 444.5 GiB (65%), /boot: 961.8 MiB / 1.9 GiB (49%)
:cccccccccccccc;MMM.;cccccccccccccccc:
:ccccccc;oxOOOo;MMM000k.;cccccccccccc:
cccccc;MMKxdd:;MMMkddc.;cccccccccccc;
ccccc;XMO';cccc;MMM.;cccccccccccccccc'
ccccc;MMo;ccccc;MMW.;ccccccccccccccc;
ccccc;MNc.ccc.xMMd;ccccccccccccccc;
cccccc;dNMWXXXWM0:;cccccccccccccc:,
```

## Install

```sh
cargo install --path .
```

## Usage

```
rfetch [OPTIONS]
```

| Option | Description |
|---|---|
| `-m, --modules <MODULES>` | Show only these modules, in the given order |
| `-u, --hide-modules <MODULES>` | Hide these from the default set |
| `--logo-off` | Drop the logo, print a single column |
| `--logo <LOGO>` | Force a logo by name or alias, or `off` |
| `--logo-list` | List all 531 available logos |
| `--no-color` | Disable ANSI colour |
| `--nerd-separator` | Use the Nerd Font separator instead of `:` |
| `--pipe` | Plain `Key: value` output for scripts |
| `--json` | JSON object output |

```sh
rfetch --modules os,ram,cpu     # custom selection and order
rfetch --hide-modules gpu        # everything except the GPU
rfetch --pipe | grep '^RAM:'     # scriptable
rfetch --json | jq .kernel       # structured output
rfetch --logo-list | grep nix    # find a logo by name
```

## Modules

`os`, `host`, `kernel`, `uptime`, `cpu`, `gpu`, `ram`, `disk`

A module whose data is unavailable is silently omitted rather than printed as
`unknown`, so the output stays clean on bare servers and inside containers.

GPU detection tries `nvidia-smi` first, then `lspci`, then `/sys/class/drm`.

## Logos

**531 logos, vendored verbatim from fastfetch** (`src/logo/ascii`, MIT — see
`LICENSE-fastfetch`). The art is stored unmodified under `logos/`, laid out in
the same `a/`, `b/`, `c/` … structure fastfetch uses.

Fastfetch's per-logo metadata lives in C headers, not in the `.txt` files, so
`tools/generate_logos.py` reads those headers and emits `src/logo/generated.rs`,
pairing each logo with its display names and colour palette:

```sh
python3 tools/generate_logos.py          # regenerate the table
python3 tools/generate_logos.py --check  # fail if the table is stale
```

Art uses `$1`, `$2`, … as palette indices. The parsing follows fastfetch's
`ffLogoPrintChars` exactly, because the details matter:

* `$` takes **one** following character, not a number — `$2888` is palette entry
  2 followed by the literal text `888`;
* `$$` is a literal `$`;
* an out-of-range index leaves the `$` as text;
* the active colour carries over to the next line.

The key column takes its colour from the logo's first palette entry, matching
fastfetch's default of `.colorKeys = colors[0]`.

A logo is chosen from the `ID` field of `/etc/os-release` and dropped
automatically when the terminal is too narrow to fit both columns.

## Notes

Colour is disabled automatically when stdout is not a TTY, when `NO_COLOR` is
set, or with `--no-color`, so redirected output never contains escape sequences.

The `cpu` module needs two samples ~200 ms apart to compute load, which accounts
for essentially all of the runtime. Runs that do not request it finish instantly.

## Development

```sh
cargo test                              # 108 unit + integration tests
cargo clippy --all-targets
cargo fmt
```

One of the integration tests renders all 531 logos and checks each one resolves.

## License

MIT. Logo artwork is © fastfetch contributors, also MIT, bundled under
`LICENSE-fastfetch`.