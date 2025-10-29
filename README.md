# Termux SU Wrapper

```text
SU wrapper for termux

Usage: tsw [OPTIONS] [COMMAND]...

Arguments:
  [COMMAND]...  Command to execute (interactive shell if omitted)

Options:
  -s, --shell <SHELL>  Shell to use with su [env: TSW_SHELL=] [default: bash]
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version

Environment variables:
  TSW_SHELL       Shell to use with SU [default: bash]
  TSW_SU_PATH     Path to su binary [default: /system/bin/su]
  TSW_HOME_ENV    Root user home directory (relative to TERMUX_FS if relative) [default: root]
```
