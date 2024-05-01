# Architecture Diagram

The basic code design dependency graph is as follows:

```
┌─────────┐
│Memcached│
└┬────────┘
┌▽───────────────────────────────┐
│TcpServer                       │
└┬─┬───────────┬────┬───────────┬┘
 │┌▽─────────┐┌▽──┐┌▽─────────┐┌▽─────────┐
 ││TcpWorker2││...││TcpWorkerN││TcpWorker1│
 │└┬─────────┘└┬──┘└┬─────────┘└┬─────────┘
 │┌▽───────────▽────▽───────────▽─┐
 ││Commands                       │
 │└┬──────────────────────────────┘
┌▽─▽──────┐
│RepoAgent│
└┬────────┘
┌▽───┐
│Repo│
└────┘
```

- `Memcached` is the main module. Use it to start our toy Memcached service.
- `TcpServer` listens on the desired port for new connections. It starts the module that owns the service's shared state, `RepoAgent`. New client connections are handed off `TcpWorker`s to serve.
- `TcpWorker` handles a single client connection. Receives and sends data from and to the client.
- `Commands` handles the Memcached service's input and output. It knows how to talk to the shared state (`RepoAgent`) and knows how to format the service's response to the client.
- `RepoAgent` acts as the manager of the Memcached's shared state (the key-value data).
- `Repo` handles the core logic of the Memcached service.
