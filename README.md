# Rfetch

A fastfetch clone written in Rust.

```
        ,.=:^!^!t3Z3z.,   OS     : Fedora Linux 44 (KDE Plasma Desktop Edition)
     :tt:::tt333EE3       Host   : babich@fedora
    Et:::ztt33EEEL        Kernel : Linux 7.2.5-200.fc44.x86_64
   ;tt:::tt333EE7         Uptime : 4h 10m
  :Et:::zt333EEQ.         CPU    : Intel(R) Xeon(R) CPU E5-2680 v3 (12) @ 2.94 GHz : 1%
  it::::tt333EEF          GPU    : NVIDIA Corporation TU116
 ;3=*^```"*4E7            RAM    : 5.4 GiB / 15.4 GiB (35%)
 :tt:::tt333EE7           Disk   : /: 289.2 GiB / 444.5 GiB (65%), /boot: 961.8 MiB / 1.9 GiB (49%)
  ;Ett:::zt333EEQ.
   ;Et:::ztt333EEF
     :tt:::tt333EE7
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
| `--logo <LOGO>` | Force a logo (`fedora`, `arch`, `nixos`, ...) or `off` |
| `--no-color` | Disable ANSI colour |
| `--nerd-separator` | Use the Nerd Font separator instead of `:` |
| `--pipe` | Plain `Key: value` output for scripts |
| `--json` | JSON object output |

```sh
rfetch --modules os,ram,cpu     # custom selection and order
rfetch --hide-modules gpu        # everything except the GPU
rfetch --pipe | grep '^RAM:'     # scriptable
rfetch --json | jq .kernel       # structured output
```

## Modules

`os`, `host`, `kernel`, `uptime`, `cpu`, `gpu`, `ram`, `disk`

A module whose data is unavailable is silently omitted rather than printed as
`unknown`, so the output stays clean on bare servers and inside containers.

GPU detection tries `nvidia-smi` first, then `lspci`, then `/sys/class/drm`.

## Logos

`fedora`, `arch`, `debian`, `ubuntu`, `mint`, `nixos`, `manjaro`, `gentoo`,
`opensuse`, `alpine`, `void`, plus a `generic` fallback. The logo is picked from
the `ID` field of `/etc/os-release` and dropped automatically when the terminal
is too narrow to fit both columns.

## Notes

Colour is disabled automatically when stdout is not a TTY, when `NO_COLOR` is
set, or with `--no-color`, so redirected output never contains escape sequences.

The `cpu` module needs two samples ~200 ms apart to compute load, which accounts
for essentially all of the runtime. Runs that do not request it finish instantly.

## Development

```sh
cargo test        # 93 unit + integration tests
cargo clippy --all-targets
cargo fmt
```

## License

MIT