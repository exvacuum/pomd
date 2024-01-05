# pomd: pomodoro daemon

[![Crates.io Version](https://img.shields.io/crates/v/pomd)](https://crates.io/crates/pomd)

This program provides a simple pomodoro daemon for linux. Recommended to be used with [pomc](https://github.com/exvacuum/pomc) client

## Features
- D-Bus interface for pomodoro functionality
- Configurable:
    - Duration of work period, short breaks, and long breaks
    - Number of iterations before long breaks
    - Enable/disable notifications

### D-Bus Interface
![image](https://github.com/exvacuum/pomd/assets/17646388/e80d9893-94b6-4450-a1c3-2e2893ca3eb7)

## Installation
### Via crates.io
```sh
cargo install pomd
```

### From Source
```sh
cargo install --path .
```

## Usage
To use the program, simply run `pomd` wherever you run startup programs. It requires a D-Bus session, so if you use xinit/startx you will need to start the program after launching your session.

The [pomc](https://github.com/exvacuum/pomc) client application can be used to interact with the daemon, or you can interact with the interface in your own scripts/programs.

## Configuration
The first time the program is run, a config.toml file will be created in your configuration directory (this is handled by the [confy](https://crates.io/crates/confy) crate, and is probably `$XDG_CONFIG_HOME/config.toml`). The keys are relatively self-explanatory, and all of the "duration" values should be specified in seconds.
