
TAPRIO on Docker
================

At the beginning, we attempted to use Docker for testing,
trying it on macOS and an Azure virtual machine running Ubuntu 20.04 with two Docker containers for simulation.
Both approaches eventually failed.
So, this section serves as a record of the troubleshooting process.

Failed on MacOS
---------------

Install docker on MacOS and try to add a IPVLAN

.. code-block:: sh

   docker network create -d ipvlan --subnet=192.168.1.0/24 --gateway=192.168.1.1 -o parent=en0 dockeripvlan1

   #Error response from daemon: invalid subinterface vlan name en0, example formatting is eth0.10

.. code-block:: sh

   docker network create -d ipvlan --subnet=192.168.1.0/24 --gateway=192.168.1.1 -o parent=en0.100 dockeripvlan1

   #Error response from daemon: -o parent interface was not found on the host: en0

The reason might be because Docker Desktop on MacOS runs a hidden Linux VM,
and that might also make the macvlan driver tricky to setup.

Ubuntu20.04
^^^^^^^^^^^

The goal is to create two Docker containers that communicate using ipvlan.
On the virtual network interface inside the containers, configure tc qdisc taprio.


* Set IPVALN

.. code-block:: sh

   docker network create -d ipvlan \
       --subnet=10.4.0.0/24 \
       --gateway=10.4.0.1 \
       -o ipvlan_mode=l2 \
       -o parent=eth0 \
       db_net


* Two docker container ubuntu:20.04

.. code-block:: sh

   docker run --net=db_net --name=apple -itd ubuntu:20.04 /bin/sh

If configuring the network only on ipvlan, the containers will not be able to connect to the external network.
Therefore, the "docker network connect" command is used to modify the available networks for a container
(by default, it is set to bridge).

.. code-block:: sh

   docker network connect bridge banana
   docker network disconnect bridge banana


* Install iproute2, nano, gcc, iputils-ping, iperf
* Use iperf to try different message sizes.

.. code-block:: sh

   iperf -c <sender_ip_address> -p <port_number> -b <bandwidth> -S <priority>

Qdisc
~~~~~

.. code-block:: sh

   tc qdisc show dev eth0@if9
   # Cannot find device "eth0@if9"
   # qdisc should be set on physical interfaces, not VLAN interfaces

   tc qdisc show dev eth0    
   qdisc noqueue 0: root refcnt 2
   # This works

.. code-block:: sh

   ip link show eth0
   7: eth0@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UNKNOWN mode DEFAULT group default 
       link/ether 60:45:bd:3a:65:45 brd ff:ff:ff:ff:ff:ff link-netnsid 0

In the container, eth0 and eth0@if2 are the same network interface.

.. code-block:: sh

   tc qdisc replace dev eth0 parent root handle 100 taprio \
                        num_tc 3 \
                        map 2 2 1 0 2 2 2 2 2 2 2 2 2 2 2 2 \
                        queues 1@0 1@0 1@0 \
                        base-time 1528743495910289987 \
                        sched-entry S 01 300000 \
                        sched-entry S 02 300000 \
                        sched-entry S 04 400000 \
                        flags 0x1 \
                        txtime-delay 200000 \
                        clockid CLOCK_TAI

Get error 

.. code-block::

   RTNETLINK answers: Operation not permitted

This error is due to privilege. Add --cap-add=NET_ADMIN.

.. code-block:: sh

   docker run --net=db_net --cap-add=NET_ADMIN --name=cherry -itd fruits:1.0 /bin/sh

The error changed to:

.. code-block:: sh

   RTNETLINK answers: Operation not supported

Other qdisc command can run properly, but taprio and mqprio doesn't work.

check channel parameter:

.. code-block:: sh

   ethtool -l eth0

Works on bridge (eth1), but error on ipvlan:

.. code-block:: sh

   Channel parameters for eth0:
   Cannot get device channel parameters
   : Operation not supported

**Taprio can be executed on the host, but not on the bridge or ipvlan in the container.**

Host

.. code-block::

   Current hardware settings:
   RX:     0
   TX:     0
   Other:      0
   Combined:   1

Container - Bridge

.. code-block::

   Current hardware settings:
   RX:     1
   TX:     1
   Other:      0
   Combined:   0

dmesg : check kernel messages

.. code-block::

   eth0: Caught tx_queue_len zero misconfig
   eth1: Caught tx_queue_len zero misconfig

But doesn't show up every time.
Another weird thing is that TX of bridge is actually not zero, and the TX of host is zero.

Need ethtool to fix the setting of TX, but it seems like the driver of ipvlan doesn't support this.

*I guess try other container or method might be easier.*
