# DatabaseFinalProject
This is a group final project for 4453 Systems: Databases
by Isabelle Hjelden, Jadyn Sondrol, and Ash Plasek

## Final Summary
For our project, we wanted to create a website that contained a list of the spells from dnd 3.5 that was filterable. There are many aspect to the spells so we decided to limit our data to requirements of the spell. People can filter search spells by name, range, duration, class, level, and components (somatic, material, and verbal). A user can also add spells to the database through the website. 

We managed to complete every goal we set for this website. We would love to continue this project by adding the rest of the information, including the descriptions of the spells and the damage caused by them. By adding the rest of the information, this website would become extremely helpful to a dnd player. Despite this, we are proud of the work we’ve done for the website.


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
