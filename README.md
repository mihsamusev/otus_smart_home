# Smart home

Collection of homework sumbmissions for course [OTUS Rust Developer](https://otus.ru/lessons/rust-developer/?int_source=courses_catalog&int_term=programming)

[![](https://github.com/mihsamusev/otus_smart_home/actions/workflows/build.yml/badge.svg)](https://github.com/mihsamusev/otus_smart_home/actions/workflows/build.yml)

Homework 14

- Crate `smart_home` - provides functionality of TCP socket device client and server
- `net_socket_emulator` -  starts a smart socket TCP server on port 8888
- `tcp_socket_ui` - an `egui` UI to turn on / off and query the status of the smart socket at given address and port

To get started:
```sh
cargo run --example net_socket_emulator # start a TCP server for a smart socket that is listenning for commands
cargo run --example tcp_socket_ui # UI for interacting with tcp smart socket
```

![](/ui.png)
