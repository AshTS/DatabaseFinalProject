# DatabaseFinalProject
This is a group final project for 4453 Systems: Databases
by Isabelle Hjelden, Jadyn Sondrol, and Ash Plasek

(README is undercontruction)

## First Time Build

When building for the first time, the web assembly target must be added via `rustup` to allow the client to build. To do this run

```
rustup target add wasm32-unknown-unknown
```

In addition, the client is served using `trunk`, which needs to be installed.

```
cargo install --locked trunk
```

## Execution

To build and host the server, run `cargo run` in the `server/` directory.

To build and host the client, run `cargo run` in the `client/` directory. 