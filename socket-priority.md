# Socket Priority on Applications

In this TSN network, the application is responsible for providing
desired priority on data packets. The standard method is to configure
the `SO_PRIORITY` option on sockets using the `setsockopt` system
call. Here are C and Rust examples.

In the C programming language, the socket is represented by a file
descriptor. The file descriptor and the priority number are provided
to `setsockopt()`.


```c
#include <sys/socket.h>

// Open a socket
int fd = socket(AF_INET, SOCK_STREAM, 0);

// Set the SO_PRIORITY to 6
int priority = 6;
int ret = setsockopt(fd, SOL_SOCKET, SO_PRIORITY, &priority, sizeof(priority));
if (ret < 0) { /* error */ }
```

In the Rust programming language, a TCP connection is created by a
`TcpStream` and the underlying file descriptor is obtained from the
stream. We call the `setsockopt()` from the
[nix](https://crates.io/crates/nix) library to configure the socket
priority.

```rust
use nix::sys::socket::{sockopt::Priority, getsockopt, setsockopt};
use std::net::TcpStream;
use std::os::unix::io::AsRawFd;

// Create a TCP socket
let mut stream = TcpStream::connect("11.22.33.44")?;

// The the underlying file descriptor of the socket.
let fd = stream.as_raw_fd();

// Set SO_PRIORITY to 6.
setsockopt(fd, Priority, 6)?;
```


## The Effect Socket Priority and Mapping

The socket priority number ranges from 0 to 15. The packets with
higher priority number are usually processed first, depending on the
actual queuing policies on the selected network device. Setting a
priority greater than 6 requires the root permission, or
`CAP_NET_ADMIN` capability to be precise.

The socket priority number is valid only within the Linux system. The
ingress or egress network devices are responsible for the translation
among the socket priorities and the _network priorities_. The network
priority could be the _Priority code point_ (PCP) field in the VLAN
header, or the _Type of Service_ field in the IPv4 header. The actual
representation depends on the configuration of the network device.
