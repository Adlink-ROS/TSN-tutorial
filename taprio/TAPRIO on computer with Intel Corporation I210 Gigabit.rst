
TAPRIO on computers with Intel Corporation I210 Gigabit
=======================================================

Setting that we are using now

Use two computers both with NIC I210 in b04.

.. code-block:: sh

   ethtool -l enP4p4s0

.. code-block::

   Channel parameters for enP4p4s0:
   Pre-set maximums:
   RX:     0
   TX:     0
   Other:      1
   Combined:   4
   Current hardware settings:
   RX:     0
   TX:     0
   Other:      1
   Combined:   4

Have 4 channels.

Some issue during experiment
----------------------------

Cannot detect interface
^^^^^^^^^^^^^^^^^^^^^^^


* The port used for the experiment cannot be detected. It is likely a hardware issue.
* After we removed the network card and switched it to another slot, then reconfigured the IP and VLAN settings.

Unable to ping the other VLAN.
Shortly after booting up, unable to ping even the physical interfaces.
Checked the routes:

.. code-block::

   192.168.1.0/24 dev vlan1 proto kernel scope link src 192.168.1.2 metric 400 linkdown 
   192.168.7.0/24 dev enP4p4s0 proto kernel scope link src 192.168.7.2 metric 102 linkdown

Display as "linkdown," manually bringing them up (using ip link set up) had no effect.
Physical interface status displayed as:

.. code-block::

   <NO-CARRIER,BROADCAST,MULTICAST,UP>

Attempts such as reconfiguring IP settings and restarting via nmtui were unsuccessful.
The issue was ultimately resolved by changing the network card to a different slot.

socket-priority experiment on 2023/08/11 and 2023/08/18
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

**Later, we discovered that the errors encountered that day were due to the failure to enable priority 0,**
**causing ARP packets not to be transmitted.**
**These are the experimental log.**

`ARP issues <./TAPRIO%20experiment/ARP%20Issues.md>`_

Calculate RTT
^^^^^^^^^^^^^

**We use RTT to verify whether the queue's activation and latency behave as expected.**

`Round Trip Time <./TAPRIO%20experiment/Round%20Trip%20Time.md>`_
