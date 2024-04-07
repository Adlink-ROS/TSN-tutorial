
TAPRIO on Azure VMs
===================

This also failed.

Create two Azure VMs (tico, tico2) and place them in the same VLAN subnet.
tico is the client, and tico2 is server.
Execute on the client port:

.. code-block:: sh

   sudo tc qdisc replace dev eth0 parent root handle 100 taprio \
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

.. code-block:: sh

   # tc qdisc show dev eth0
   qdisc taprio 100: root refcnt 65 tc 3 map 2 2 1 0 2 2 2 2 2 2 2 2 2 2 2 2
   queues offset 0 count 1 offset 0 count 1 offset 0 count 1
   clockid TAI flags 0x1 txtime delay 200000   base-time 1528743495910289987 cycle-time 1000000 cycle-time-extension 0
       index 0 cmd S gatemask 0x1 interval 300000
       index 1 cmd S gatemask 0x2 interval 300000
       index 2 cmd S gatemask 0x4 interval 400000

Set three priority in iperf

.. code-block::

   ------------------------------------------------------------
   Client connecting to 10.0.0.5, TCP port 5001
   TCP window size:  230 KByte (default)
   ------------------------------------------------------------
   [  3] local 10.0.0.4 port 48656 connected with 10.0.0.5 port 5001
   [ ID] Interval       Transfer     Bandwidth
   [  3]  0.0- 5.4 sec  6.88 MBytes  10.6 Mbits/sec

Either add tc or not the three priority remains the same.
This is because ``queues 1@0 1@0 1@0`` means the three priority goes in the same queue.

.. code-block:: sh

   sudo tc qdisc replace dev eth0 parent root handle 100 taprio \
                        num_tc 3 \
                        map 1 2 3 0 0 0 0 0 0 0 0 0 0 0 0 0 \
                        queues 1@0 1@1 1@2 \
                        base-time 200 \
                        sched-entry S 00 300000 \
                        sched-entry S 02 300000 \
                        sched-entry S 04 800000 \
                        flags 0x1 \
                        txtime-delay 200000 \
                        clockid CLOCK_TAI

Get error

.. code-block::

   Changing the traffic mapping of a running schedule is not supported.

Just delete the qdisc setting.

.. code-block::

   Invalid traffic class in priority to traffic class mapping.

This is because *num_tc 3* so the number of map can only set to 2.

.. code-block::

   1@0 2@0 1@0
   1@0 1@1 1@0

both settings can't work.

.. code-block::

   Invalid queue in traffic class to queue mapping.

.. code-block:: sh

   ethtool -l eth0
   Channel parameters for eth0:
   Pre-set maximums:
   RX:             0
   TX:             0
   Other:          0
   Combined:       1
   Current hardware settings:
   RX:             0
   TX:             0
   Other:          0
   Combined:       1

The NIC only has 1 queue.
So we can't use more than 1 queue.
