# Memcached

This is a toy Memcached server/service based on a [coding challenge](https://codingchallenges.fyi/challenges/challenge-memcached).

The goal was to learn about how Elixir GenServers, Tasks, and Agents interacted with TCP connections.

## Development

Use [devbox](https://github.com/jetpack-io/devbox) to create your dev shell:

```
$ devbox shell
```

Once the shell starts, you should have the elixir available.
Edit the code in your choice editor. Come back to the dev shell to compile, test, etc.
`mix` is your friend.

### Unit tests

```
$ mix test
```

### Testing manually with an actual TCP connection

You'll need two dev shells open: one to start the Memcached server (S1), the other to connect as a client (S2).

In S1, start up the server via `iex` and run `Memcached.start()`:

```
$ iex -S mix

Erlang/OTP 25 [erts-********] [source] [64-bit] [smp:10:10] [ds:10:10:10] [async-threads:1]

Interactive Elixir (1.16.2) - press Ctrl+C to exit (type h() ENTER for help)
iex(1)> Memcached.start()
Accepting connection on port: 11211
```

In S2, use `telnet` to connect to the server. Telnet is automatically installed in your dev shell (just like how Elixir was already available).

The following example connects to the server, adds (`set`) a key-value (called "key" with value of "hello"), retrieves (`get`) for that same key, and then disconnects from the server.

```
$ telnet localhost 11211
Trying 127.0.0.1
Connected to localhost.
Escape character is '^]'.
set key 1 0 5
hello
STORED
get key
VALUE key 1 5
hello
END
^]
telnet> quit
Connection closed.
```

On the server (S1), you should see the following debug messages showing that the key-value pair was stored and retrieved:

```
Erlang/OTP 25 [erts-********] [source] [64-bit] [smp:10:10] [ds:10:10:10] [async-threads:1]

Interactive Elixir (1.16.2) - press Ctrl+C to exit (type h() ENTER for help)
(search)`start': Memcached.start()
Accepting connection on port: 11211
Client connected
*DBG* <0.137.0> got {tcp,#Port<0.7>,<<"set key 1 0 5\r\n">>}
*DBG* <0.137.0> new state {#Port<0.7>,
                           [<<"set">>,<<"key">>,<<"1">>,<<"0">>,<<"5">>],
                           5,[]}
*DBG* <0.137.0> got {tcp,#Port<0.7>,<<"hello\r\n">>}
*DBG* 'Elixir.Memcached.RepoAgent' got call {update,
                                             #Fun<Elixir.Memcached.RepoAgent.1.105515450>} from <0.137.0>
*DBG* 'Elixir.Memcached.RepoAgent' sent ok to <0.137.0>,
  new state #{'__struct__' =>
       'Elixir.Memcached.Repo',
          storage =>
       #{<<"key">> =>
             #{'__struct__' =>
                   'Elixir.Memcached.Value',
               byte_count =>
                   5,
               created_ts =>
                   #{'__struct__' =>
                         'Elixir.DateTime',
                     calendar =>
                         'Elixir.Calendar.ISO',
                     day =>
                         20,
                     hour =>
                         7,
                     microsecond =>
                         {871896,
                          6},
                     minute =>
                         11,
                     month =>
                         4,
                     second =>
                         29,
                     std_offset =>
                         0,
                     time_zone =>
                         <<"Etc/UTC">>,
                     utc_offset =>
                         0,
                     year =>
                         2024,
                     zone_abbr =>
                         <<"UTC">>},
               data =>
                   [<<"hello">>],
               exp_time =>
                   0,
               flags =>
                   1}}}
*DBG* <0.137.0> new state #Port<0.7>
*DBG* <0.137.0> got {tcp,#Port<0.7>,<<"get key\r\n">>}
*DBG* 'Elixir.Memcached.RepoAgent' got call {get_and_update,
                                             #Fun<Elixir.Memcached.RepoAgent.0.105515450>} from <0.137.0>
*DBG* 'Elixir.Memcached.RepoAgent' sent {ok,
                                         #{'__struct__' =>
                                            'Elixir.Memcached.Value',
                                           byte_count => 5,
                                           created_ts =>
                                            #{'__struct__' =>
                                               'Elixir.DateTime',
                                              calendar =>
                                               'Elixir.Calendar.ISO',
                                              day => 20,hour => 7,
                                              microsecond => {871896,6},
                                              minute => 11,month => 4,
                                              second => 29,std_offset => 0,
                                              time_zone => <<"Etc/UTC">>,
                                              utc_offset => 0,year => 2024,
                                              zone_abbr => <<"UTC">>},
                                           data => [<<"hello">>],
                                           exp_time => 0,flags => 1}} to <0.137.0>,
                                           new state #{'__struct__' =>
                                               'Elixir.Memcached.Repo',
                                             storage =>
                                               #{<<"key">> =>
                                                     #{'__struct__' =>
                                                           'Elixir.Memcached.Value',
                                                       byte_count =>
                                                           5,
                                                       created_ts =>
                                                           #{'__struct__' =>
                                                                 'Elixir.DateTime',
                                                             calendar =>
                                                                 'Elixir.Calendar.ISO',
                                                             day =>
                                                                 20,
                                                             hour =>
                                                                 7,
                                                             microsecond =>
                                                                 {871896,
                                                                  6},
                                                             minute =>
                                                                 11,
                                                             month =>
                                                                 4,
                                                             second =>
                                                                 29,
                                                             std_offset =>
                                                                 0,
                                                             time_zone =>
                                                                 <<"Etc/UTC">>,
                                                             utc_offset =>
                                                                 0,
                                                             year =>
                                                                 2024,
                                                             zone_abbr =>
                                                                 <<"UTC">>},
                                                       data =>
                                                           [<<"hello">>],
                                                       exp_time =>
                                                           0,
                                                       flags =>
                                                           1}}}
*DBG* <0.137.0> new state #Port<0.7>
```
