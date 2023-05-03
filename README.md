# DatabaseFinalProject
This is a group final project for 4453 Systems: Databases
by Isabelle Hjelden, Jadyn Sondrol, and Ash Plasek

(README is undercontruction)

## Requirements

This project uses mongodb for the database service, and thus the mongodb backend must be running on the host machine on port `27017`. 

Additionally, Rust and `cargo` must be installed using `rustup`. 

Python 3 is required to run the data generation scripts if pre generated data is required.

## First Time Build

When building for the first time, the web assembly target must be added via `rustup` to allow the client to build. To do this run

```
rustup target add wasm32-unknown-unknown
```

In addition, the client is served using `trunk`, which needs to be installed.

```
cargo install --locked trunk
```

Finally, the database must be generated, or it can be populated from scratch. To generate the csv files to populate it, run the `load.py` and then `generate.py` files in the `data` directory.

The `class_table.csv` file should be imported into the `local/class_table` database, and the `spell_table.csv` file should be imported into the `local/spell_table` database if the generated data is desired. 

## Execution

To build and host the server, run `cargo run` in the `server/` directory.

To build and host the client, run `cargo run` in the `client/` directory. 