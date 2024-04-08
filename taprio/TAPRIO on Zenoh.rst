
Taprio on Zenoh
===============

Note: This experiment was not successful,  the throughput did not meet expectations.

Code Repository
---------------

We used the modified Zenoh which support socket-priority. 
https://github.com/NEWSLabNTU/zenoh.git

Target
^^^^^^

Compare the overhead of dividing different priorities into multiple processes (scheduled by the OS) with 1 process (a Zenoh process, as spawning processes also incurs costs).

Originally, one priority corresponded to one process, now changed to multiple priorities corresponding to one process.
Measure the priority switching within the same process.

The original Zenoh example priority tests
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block::

   examples/examples/z_sub_thr.rs
   examples/examples/z_pub_thr.rs

The multi-process Zenoh example priority tests
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block::

   examples/examples/z_sub_thr_p.rs
   examples/examples/z_pub_thr_s.rs

Implementation Idea
^^^^^^^^^^^^^^^^^^^

For the publisher, open multiple sessions with different priorities in multi-threaded mode, each session having a different topic to distinguish different priorities. Similarly, for the subscriber, open sessions with different priorities in multi-threaded mode. Set up TAPRIO on the publisher side and measure the throughput for different priorities on the subscriber side.

.. image:: ./TAPRIO\ experiment/images/ZenohTSN.png
   :target: ./TAPRIO\ experiment/images/ZenohTSN.png
   :alt: image

