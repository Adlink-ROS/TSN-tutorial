
VLAN and Priority Translation
=============================

The translation process is necessary to reserve Linux socket priority in the network during hop-to-top data transmission.
The *socket priority* is only visible within a Linux system.
When a packet goes outbound from a network device, the priority number is translated and is recorded in the packet header.
The header can be either a PCP field in VLAN or the ToS field in IPv4.
In this article, the VLAN PCP is used.

A VLAN Network Example
----------------------

To enable VLAN PCP tagging, a virtual VLAN device is created atop of a physical device with an assigned IP address.
Packets going out from the VLAN device are encapsulated with a VLAN header.
Let's start with a network of Alice and Bob.

.. mermaid::

   flowchart TB
       subgraph peer2 ["Bob"]
           direction TB

           peer2-vlan2["vlan2
           192.168.2.2"]
           peer2-eth0["eth0
           10.8.0.2"]

           peer2-vlan2 --- peer2-eth0
       end

       subgraph peer1 ["Alice"]
           direction TB

           peer1-vlan2["vlan2
           192.168.2.1"]
           peer1-eno1["eno1
           10.8.0.1"]

           peer1-vlan2 --- peer1-eno1
       end

       phy("Local Network
       10.8.0.0/24")

       phy --- peer2-eth0
       phy --- peer1-eno1

In the following sections, we will go through these steps to enable VLAN tagging.


#. Identify the backend physical device.
#. Create a VLAN device.
#. Configure the priority mapping.

Identify the Backend Physical Device
------------------------------------

Alice and Bob are connected through a local Ethernet network.
Run ``ip address`` on Alice and Bob sides and you'll see this.

On Alice's computer,

.. code-block::

   1: eno1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
       link/ether ff:ff:ff:ff:ff:ff brd ff:ff:ff:ff:ff:ff
       inet 10.8.0.1/24 brd 10.8.0.255 scope global dynamic noprefixroute eno1
          valid_lft 5300sec preferred_lft 5300sec

On Bob's computer,

.. code-block::

   1: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
       link/ether ff:ff:ff:ff:ff:ff brd ff:ff:ff:ff:ff:ff
       inet 10.8.0.2/24 brd 10.8.0.255 scope global dynamic noprefixroute eno1
          valid_lft 5300sec preferred_lft 5300sec

Both of them physically connected on a local network, prefixed with IP ``10.8.0.x``.
Now, we'll start a virtual local-area network (VLAN), prefixed with ``192.168.2.x``.
It works as though Alice and Bob are connected through a virtual hub.
To do so, we'll create a virtual network device named ``vlan2`` on both sides.
It's named ``vlan2`` because they share the VLAN ID 2.

Create VLAN Devices
-------------------

On the Alice's side, run ``sudo nmtui`` → Edit a connection → Add → Select VLAN and click Create.
Fill in the form like this.
Bob will do so similarly but with a different IP.

.. code-block::

   ┌───────────────────────────┤ Edit Connection ├───────────────────────────┐
   │                                                                         │
   │         Profile name VLAN2___________________________________           │
   │               Device vlan2___________________________________           │
   │                                                                         │
   │ ╤ VLAN                                                        <Hide>    │
   │ │             Parent eno1____________________________________           │
   │ │            VLAN id 2_______                                           │
   │ │                                                                       │
   │ │ Cloned MAC address ________________________________________           │
   │ │                MTU __________ bytes                                   │
   │ └                                                                       │
   │                                                                         │
   │ ╤ IPv4 CONFIGURATION <Manual>                                 <Hide>    │
   │ │          Addresses 192.168.2.1/24___________ <Remove>                 │
   │ │                    <Add...>                                           │
   │ │            Gateway _________________________                          │
   │ │        DNS servers <Add...>                                           │
   │ │     Search domains <Add...>                                           │
   │ │                                                                       │
   │ │            Routing (No custom routes) <Edit...>                       │
   │ │ [X] Never use this network for default route                          │
   │ │ [ ] Ignore automatically obtained routes                              │
   │ │ [ ] Ignore automatically obtained DNS parameters                      │
   │ │                                                                       │
   │ │ [ ] Require IPv4 addressing for this connection                       │
   │ └                                                                       │
   │                                                                         │
   │ ═ IPv6 CONFIGURATION <Automatic>                              <Show>    │
   │                                                                         │
   │ [X] Automatically connect                                               │
   │ [X] Available to all users                                              │
   │                                                                         │
   │                                                           <Cancel> <OK> │
   └─────────────────────────────────────────────────────────────────────────┘

Let's ``ip address`` again.
You can see the newly created network device on the Alice's side.

.. code-block::

   2: vlan2@eno1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
       link/ether ff:ff:ff:ff:ff:ff brd ff:ff:ff:ff:ff:ff
       inet 192.168.2.1/24 brd 192.168.2.255 scope global noprefixroute vlan2
          valid_lft forever preferred_lft forever

Configure the Priority Mapping
------------------------------

Now here comes the crucial part.
We have to tell the ``vlan2`` interface to map socket priority 0 to PCP 0, socket priority 1 to PCP 1, and so on.
The command goes like this.
The ``0:0`` term means mapping the socket priority 0 on the left of colon to PCP 0 on the right.

.. code-block:: sh

   sudo ip link \
       set dev vlan2 \
       type vlan \
       egress 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7

To testify our settings, we run the socket priority testing program from the previous section and set the sender priority to 6.
Run the ``tcpdump`` packet sniffer and packets captured will show that PCP=6.
(default value of pcp is 0)

.. code-block::

   18:06:30.891525 08:26:97:f7:49:c5 (oui Unknown) > 08:26:97:f7:49:c9 (oui Unknown), ethertype 802.1Q (0x8100), length 2966: vlan 1, p 6, ethertype IPv4 (0x0800), (tos 0x0, ttl 64, id 23093, offset 0, flags [DF], proto TCP (6), length 2948)
       192.168.1.2.36196 > ros-RSK.55555: Flags [P.], cksum 0x8eca (incorrect -> 0x27f2), seq 76361265:76364161, ack 0, win 502, options [nop,nop,TS val 3959431146 ecr 1510782482], length 2896

A Little Summary
----------------

This section brought you to set up a VLAN device and enable the priority mapping.
That's the first step to dive into Time-Sensitive network.
In later parts, we will create scheduling rules making use of these priorities.
TAPRIO will be our first example.
