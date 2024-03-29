Time-Sensitive Network Tutorial
===============================

Authors: Chung-Chi Wang, Lin Hsiang-Jui, ChenYing Kuo

This tutorial helps you enable Time-Sensitive Network (TSN) on Linux in practice.
It consists of two parts.

* One is to enable applications to mark priority numbers on packets to be sent.
* The second is to configure scheduling policies on ingress and egress network devices on Linux.

In this way, the packets with priority marks passing by these interfaces are prioritized to achieve bounded-time delivery.

- [Socket Priority on Applications](socket-priority.md)
- [VLAN and Priority Translation](priority-translation.md)
- [TAPRIO settings](taprio-settings.md)
- [iPerf3 TSN Tutorial](iperf3-tsn.md)

Contents
--------

.. toctree::

