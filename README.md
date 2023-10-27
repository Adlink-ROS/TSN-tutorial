# Time-Sensitive Network Tutorial

This tutorial helps you enable Time-Sensitive Network (TSN) on Linux
in practice. It consists of two parts. One is to enable applications
to mark priority numbers on packets to be sent. The second is to
configure scheduling policies on ingress and egress network devices on
Linux. In this way, the packets with priority marks passing by these
interfaces are prioritized to achieve bounded-time delivery.


- Using Socket Priorities in Applications
  - [Socket Priority on Applications](socket-priority.md)
  - [Priority Translation](socket-priority.md)

- Network Device Configuration
  - [TAPRIO settings](TAPRIO%20settings.md)
  - [TAPRIO on Azure VMs](TAPRIO%20on%20Azure%20VMs.md)
  - [TAPRIO on Kata Container](TAPRIO%20on%20Kata%20Container.md)
  - [TAPRIO on Docker](TAPRIO%20on%20Docker.md)
  - [TAPRIO on computer with Intel Corporation I210
    Gigabit](TAPRIO%20on%20computer%20with%20Intel%20Corporation%20I210%20Gigabit.md)

<!-- We tried several environment configurations for testing TAPRIO, and -->
<!-- the following links document the process of our attempts. In the end, -->
<!-- only [TAPRIO on computer with Intel Corporation I210 Gigabit] works, -->
<!-- and it's the configuration we've been using to date. -->

https://github.com/jerry73204/socket-priority/tree/papaya

