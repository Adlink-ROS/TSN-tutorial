
Method of Measuring Round Trip Time
===================================

iperf3
------

On one end, open the server:

.. code-block::

   iperf -S

On the other end, open the client:

.. code-block::

   iperf3 -c 192.168.1.2 -p 5001 -b 10M -S 1


* ``-c``\ : This option specifies the client mode.
* ``-p``\ : The server will listen for client connections on this port.
* ``-b``\ : This option specifies the bandwidth for the test traffic.
* ``-S``\ : This option specifies a single traffic session, meaning the test will use a single TCP or UDP connection to evaluate bandwidth and performance.

However, it's important to note that iperf measures traffic within a 1-second window,
so it may not always provide a reliable reference.


* ``-i``\ : Setting the send interval, but it only allows values between 0.1 and 60 seconds, which didn't meet our needs

Socket-Priority
---------------

`Socket-Priority <https://github.com/jerry73204/socket-priority/tree/papaya>`_
