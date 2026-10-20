---
title: Report a problem
description: What to include and where to post when DesignAdobado misbehaves or crashes.
order: 20
---

Open an [issue in the DesignAdobado tracker](https://github.com/smouj/DesignAdobado/issues) and attach the log file. The log is the most useful thing you can send after a crash.

## What to include

- Your DesignAdobado version and how you installed it (installer, portable, AppImage and so on).
- Your operating system and graphics card.
- What you did, what you expected and what happened.
- The log file `designcraft.log`.

## Where the log is

DesignAdobado writes `logs/designcraft.log` in its settings folder, beside `ui.json` and `prefs.json`.

| System  | Log folder                                                              |
|---------|-------------------------------------------------------------------------|
| Windows | `%APPDATA%\DesignAdobado\logs\`                                           |
| macOS   | `~/Library/Application Support/DesignAdobado/logs/`                       |
| Linux   | `~/.config/designcraft/logs/` (or `$XDG_CONFIG_HOME/designcraft/logs/`) |

Each start moves the previous log to `designcraft.1.log`, and that one to `designcraft.2.log`. The log from a run that crashed survives the next start, so attach `designcraft.1.log` if you have restarted since. The file stops growing at 16 MiB.

## More detail in the log

Set the environment variable `RUST_LOG` to `debug` before starting DesignAdobado to log more. The default is `info` for DesignAdobado's own parts and `warn` for everything else.
