
iPerf3 TSN Tutorial
===================

iPerf3 with support for SO_PRIORITY socket option
-------------------------------------------------

Compile
^^^^^^^

This is an iPerf3 version that provides the ``SO_PRIORITY`` option.
Before use, it needs to be compiled following the steps below.

`iPerf3 with SO_PRIORITY <https://github.com/olerem/iperf/tree/so_priority>`_


#. Run ``./configure``
#. In ``src/iperf_config.h``\ , add ``#define HAVE_SO_PRIORITY 1``
#. ``make``

Option
^^^^^^

The functionality is similar to the main iPerf3. For the ``SO_PRIORITY`` functionality, use the ``--sock-prio`` option.
The packet is sent in TCP by default, use the option ``--u`` to send in UDP.

Settings
^^^^^^^^

Two computers are connected via VLAN on their respective Controller I225-V network cards.
This is a network card that supports hardware TSN (Time-Sensitive Networking) functionality.
For the VLAN settings, check `VLAN and Priority Translation <https://github.com/Adlink-ROS/TSN-tutorial/blob/main/priority-translation.md>`_.
One computer serves as the server, and the other as the client.

Experiment
^^^^^^^^^^

By default, iPerf3 sends packets from the client to the server. Using the ``--r`` option reverses this behavior.
Therefore, when collecting data, there are a few points to note:


#. Observe the data from the receiver.
#. TAPRIO settings are configured on the sender.

Example
^^^^^^^

Without reverse option, client->server, UDP

Client
""""""

.. code-block:: bash

   seq 1 3 | parallel -j0 './src/iperf3 -c 192.168.1.1 -p 5555{} -b10G  -l1472 -t100 --sock-prio {} >./client_log/p{}_client.out'

Set TAPRIO on client

.. code-block:: bash

   sudo tc qdisc replace dev enp5s0 parent root handle 100 taprio \
        num_tc 4 \
        map 0 1 2 3 2 2 2 2 2 2 2 2 2 2 2 2 \
        queues 1@0 1@1 1@2 1@3\
        base-time 0 \
        sched-entry S 03 1000000 \
        sched-entry S 05 3000000 \
        sched-entry S 09 5000000 \
        flags 0x2

The TAPRIO configuration specifies that sockets 0 to 3 will correspond to queues 0 to 3, respectively.
In the "sched-entry" field, 03, 05, 09 respectively represent the simultaneous opening of gate 1, 2, and 3 with gate 0.
The subsequent numbers indicate the duration for which each gate remains open within one cycle.
In this example, the gates of the queues corresponding to flows with priorities 1, 2, and 3 will open with a ratio of 1:3:5.

Server
""""""

.. code-block:: bash

   seq 1 3 | parallel -j0 'iperf3 -s -p 5555{} >./server_log/p{}_server.out'

Running three clients with priorities 1, 2, and 3 simultaneously, along with their corresponding port servers, will result in transmission traffic approximately in the ratio of 1:2:3.
