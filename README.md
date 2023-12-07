# ping-pong-containers

## how to use

In one terminal, run `ping`

```
cargo run --bin ping
```

In a second terminal, run `pong`

```
cargo run --bin pong
```

In a third terminal, kick off the process by sending a message to one of them

```
curl -v -X POST 127.0.0.1:7878/pong
```

Watch them play `ping` / `pong` in the first two terminals.