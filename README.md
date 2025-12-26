# Termux SU Wrapper

[![License: GPL-3.0-only](https://img.shields.io/badge/License-GPL--3.0--only-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)  

## Installing

```bash
apt install rust
cargo install tsw
# And add '~/.cargo/bin' in $PATH
```

## Supported

- Magisk
- KernelSU Next

### Unsupported

- APatch - Not tested.
- SukiSU Ultra - Not tested.
- KernelSU - Out of scope. 
- SuperSU - Out of scope.

## Example

```bash
# Run interactive shell
tsw

# Run command in interactive shell
tsw nvim /data/adb/service.d/service.sh

# Run with fish
tsw --shell fish

# Run with master namespace
tsw --mount-master
```

## Config

Config location on  `/data/data/com.termux/files/usr/etc/tsw.toml`.
```toml
# Path to SU file (only absolute)
su_file = "/system/bin/su"
# Path to home directory for root session (only absolute)
home_dir = "/data/data/com.termux/files/root"
# $PATH variable for root session
path_env = "/data/data/com.termux/files/usr/bin:/system/bin:/debug_ramdisk:/sbin:/sbin/su:/su/bin:/su/xbin:/system/bin:/system/xbin"
# Use master namespace
mount_master = false
```

## License

The project is licensed under the [GPL 3.0 License](LICENSE).
