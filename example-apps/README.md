# Example applications

## dpp-threads

The Dining philosopher problem is implemented with quantum leaps like engine.
It runs in threaded context on host.

Run with
```sh
RUST_LOG=Info cargo run --bin dpp-threads
```

Alternatively set the log to `RUST_LOG=example_apps::dpp=Debug` to see also debug log output
on the state machine module only.

The DPP state machine runs forever. Stopping it is possible with `CTRL-C`.
