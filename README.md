# volo-mini-redis

A simple mini-redis implementation in Rust. With VOLO.

## Usage

first build the binary and set up the cli:

```bash
cargo build
source make_cli.sh
```

then start the server:

```bash
server -p [port] [ master | slave [master_port] | proxy ]
```

use client to connect to the server:

```bash
client -p [port]  [set key value [--expire sec]|  get key |  del key |  ping [key] | subscribe channel -a [other channnel] | publish  channel message ]
```

## Shell scripts

`start.sh` is used to start a master-slave servers, the conf is read from `server.conf.bak`. The format is :
```
[master port]
[slave1 port]
[slave2 port]
...

```
The last blank line is needed.
To start up a cluster. first write config in `server.conf` like this:
```
8000:
    8100+2
    8200+2
    8300+2
```
8000 will be the proxy port, 8100 will be a master with 8101 8102 as its slaves, 8200 will be a master with 8201 8202 as its slaves, and so on. 
`parse | ./target/debug/parse_conf` is used to  parse the conf to generate `begin.sh` to start all the servers, including proxy.

`test_AOF|master_slave|cluster|multi|watch.sh` is used to test each the functions of the server.
`test_all.sh` will run all the tests.

change src/arg.rs to set the proper default port for client and server. Now all 8000 

## Features

- Graceful Exit
- Transaction

For more info, please read the ppt.

