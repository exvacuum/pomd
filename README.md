# pomd: pomodoro daemon

This program provides a simple pomodoro daemon for linux.

## Features
- D-Bus interface for pomodoro functionality
- Configurable:
    - Duration of work period, short breaks, and long breaks
    - Number of iterations before long breaks
    - Enable/disable notifications

## Installation
### From Source
```sh
cargo install --path .
```

## Usage
To use the program, simply run `pomd` wherever you run startup programs. It requires a D-Bus session, so if you use xinit/startx you will need to start the program after launching your session.

The [pomc](https://github.com/exvacuum/pomc) client application can be used to interact with the daemon, or you can interact with the interface in your own scripts/programs.

## Configuration
The first time the program is run, a config.toml file will be created in your configuration directory (this is handled by the [confy](https://crates.io/crates/confy) crate, and is probably `$XDG_CONFIG_HOME/config.toml`). The keys are relatively self-explanatory, and all of the "duration" values should be specified in seconds.
