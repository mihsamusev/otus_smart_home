# Smart home

Collection of homework sumbmissions for course [OTUS Rust Developer](https://otus.ru/lessons/rust-developer/?int_source=courses_catalog&int_term=programming)

[![](https://github.com/mihsamusev/otus_smart_home/actions/workflows/build.yml/badge.svg)](https://github.com/mihsamusev/otus_smart_home/actions/workflows/build.yml)

14 homeworks are organized into branches:

- [x] [`homework1`](https://github.com/mihsamusev/otus_smart_home/tree/homework1) - Smart home prototype of 2 devices: socket and termometer
- [x] [`homework2`](https://github.com/mihsamusev/otus_smart_home/tree/homework2) - Smart home layout and device info providers
- [x] [`homework3`](https://github.com/mihsamusev/otus_smart_home/tree/homework3) - Smart home library split into modules and covered by tests
- [x] [`homework4`](https://github.com/mihsamusev/otus_smart_home/tree/homework4) - Custom errors and exception flow is added to the library
- [x] [`homework5`](https://github.com/mihsamusev/otus_smart_home/tree/homework5) - Library is refactored so that the smart home owns all device entities instead of keeping track of their id.
- [x] [`homework6`](https://github.com/mihsamusev/otus_smart_home/tree/homework6) - Smart socket can be interacted with by TCP
- [x] [`homework7`](https://github.com/mihsamusev/otus_smart_home/tree/homework7) -
- [ ] [`homework8`](https://github.com/mihsamusev/otus_smart_home/tree/homework8) -
- [ ] [`homework9`](https://github.com/mihsamusev/otus_smart_home/tree/homework9) -
- [ ] [`homework10`](https://github.com/mihsamusev/otus_smart_home/tree/homework10) -
- [ ] [`homework11`](https://github.com/mihsamusev/otus_smart_home/tree/homework11) -
- [ ] [`homework12`](https://github.com/mihsamusev/otus_smart_home/tree/homework12) -
- [ ] [`homework13`](https://github.com/mihsamusev/otus_smart_home/tree/homework13) -
- [ ] [`homework14`](https://github.com/mihsamusev/otus_smart_home/tree/homework14) -

# Examples for this submission

Example of network communication between smart socket and smart home

```sh
cargo run --example net_socket_emulator # start a TCP server on 8888 for a smart socket that is listenning for commands
cargo run --example net_termo_emulator # start a UDP server on 9000 that constantly sends data to port 9001

# can be run together with 2 client options, normal and interactive
cargo run --example net_home # non-interactive client
cargo run --example net_home_interactive # interactive home client, can query both devices
```

In addition on linux `netcat` can be used to test the servers:

```sh
nc localhost 8888 # start sending messages to TCP SmartSocket server
nc -u localhost 9000 # start sending messages to UDP SmartThermometer server
```

In addition example from previous submissions

```sh
cargo run --example mock_devices # run exaple of report generation of non-networked smart device mocks
```
