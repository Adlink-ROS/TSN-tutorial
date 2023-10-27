# Priority Translation

The translation process is necessary to reserve Linux socket priority
in the network during hop-to-top data transmission. The _socket
priority_ is configured and is only visible within a Linux
system. When a packet goes outbound from a network device, the
priority number is recorded in the packet header. The header can be
either a PCP field in VLAN or the ToS field in IPv4. In this section,
the VLAN is used.

To make use of VLAN PCP, a virtual VLAN network device should be
created and an IP address is assigned to the device.

TODO: VLAN interface creation


```sh
sudo ip link \
    set dev vlan1 \
    type vlan \
    egress 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7
```
