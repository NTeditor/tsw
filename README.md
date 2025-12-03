# Termux SU Wrapper
[![License: GPL-3.0-only](https://img.shields.io/badge/License-GPL--3.0--only-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)  

```text
SU wrapper for termux

Usage: tsw [OPTIONS] [COMMAND]...

Arguments:
  [COMMAND]...  Command to execute (interactive shell if omitted)

Options:
  -s, --shell <SHELL>                Shell to use with su [default: bash]
  -m, --mount-master <MOUNT_MASTER>  Force run in the global namespace [possible values: true, false]
  -c, --config <CONFIG>              Path to config file [default: /data/data/com.termux/files/usr/etc/tsw.toml]
  -h, --help                         Print help
  -V, --version                      Print version
```
