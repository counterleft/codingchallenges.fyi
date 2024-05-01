# Redis Server

This is a toy Redis server based on a [coding challenge](https://codingchallenges.fyi/challenges/challenge-redis).

The goal was to learn about Rust in a trivial and non-trivial way; the Redis RESP spec is well-defined, my knowledge of Rust is not.

## Development

I'm assuming `fish` as the default shell.

Use [devbox](https://github.com/jetpack-io/devbox) to create your dev shell:

```
$ devbox shell
```

Once the shell starts, rust, cargo, and redis are available.

### Unit testing

Use cargo to run the tests:

```
$ cargo test
```

### Manual testing with an actual redis client

Use `redis-cli` (it's already installed with the dev shell) to connect to our redis-server.

Open a terminal for our server:

```
$ cargo build; target/debug/redis_server
```

Open a second terminal to connect to the server with a redis client and send over a command.
The following example sends the `ECHO` redis command to the server:

```
$ redis-cli echo hello world
```
