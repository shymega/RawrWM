Notes for RawrWM
================

# Backends

- Wayland.
- X11.

Why support X11?
    - Allows switching between Wayland and X11 when necessary.
    - Also makes the WM flexible.

# Daemon

- Daemon runs on startup, entrypoint.
- rawrctl is the control interface to the daemon. Configured by shell script, pass
commands via rawrctl to daemon.
- RPC via JSON-RPC
    - JSON-RPC schemas and protocol to be designed.
- Async.

# Features to support

- HiDPI
- Speed
- Startup, fast
- Greetd support

# Configuration

- A la bspwm, configured by rawrctl.
- Configuration can be saved.
- Configuration reload on-the-fly
