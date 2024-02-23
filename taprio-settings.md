# TAPRIO settings

In the previous article, we learned to set up VLAN network and priority mapping.
Now we're going further.
We'll create a **qdisc** rule to control the traffic.
There are various kinds of qdisc rules, and this time we'll focus on TAPRIO.
This rule will respect the PCP marked on the packets provided by VLAN.

Note that it's not necessary to use VLAN to read packet priority.
802.1Q uses VLAN for this purpose.
Therefore, we configure the relationship between VLAN egress PCP and the SO\_PRIORITY value.
You can learn the details in TODO section.

## Check Hardware Queues on Network Card

The number of queues used on TAPRIO qdisc rule cannot exceed the number of queues on the network card.
Run this command to check the number of queues available on a network card, where "enp2s0f1" is the device file.

```sh
ethtool -l enp2s0f1
```

It shows 16 avaible queues for example.

```
Channel parameters for enp2s0f1:
Pre-set maximums:
RX:             n/a
TX:             n/a
Other:          1
Combined:       16
Current hardware settings:
RX:             n/a
TX:             n/a
Other:          1
Combined:       16
```

### Experimental Setup

Let's start an experimental with a server and a client, both connected through a 10Gbps wire directy.
Suppose server network device is `ens1` and client's is `enp3s0`.
Both devices have priority mapping configured like this.

```sh
sudo ip link set dev vlan1 type vlan egress 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7
```


## Create a TAPRIO Rule

Let's add a TAPRIO qdisc rule on the client side.
Note that the rule is added to the physical network device `enp3s0`, not the virtual `vlan1` device.

```sh
sudo tc qdisc replace dev enp3s0 parent root handle 100 taprio \
     num_tc 3 \
     map 0 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 \
     queues 1@0 1@1 1@2 \
     base-time 200 \
     sched-entry S 00 300000 \
     sched-entry S 02 300000 \
     sched-entry S 04 800000 \
     flags 0x1 \
     txtime-delay 200000 \
     clockid CLOCK_TAI
```

where

- `num_tc`: number of traffic class
- `map`: The relationship between Linux priority and traffic class.


- `queue`: The queues in "count@offset" notation specifies the queue
  range for each traffic class. According to the the official
  documents, the ranges should not overlap and must be a contiguous
  range of queues.

- `flags` configures the mode of operation.  It can be 0x1 or
  0x2. Enabling the execution of the Task Admission Control (TAS)
  functionality either in software or hardware.

In one of the examples provided in the documentation,
there is a configuration where flows with different priorities are directed to the same queue and executed in software.

```sh
# tc qdisc replace dev eth0 parent root handle 100 taprio \
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
```

If this configuration is using mode 0x2,
it is indeed possible for them to share the same queue.


## How does Priorty Mapping Work on Linux?

In Linux, the implementation of qdisc is based on reading a data structure called the Socket Buffer (`sk_buff`).
The system reads the priority value from `skb->priority`.
This means that regardless of the type of packet being transmitted,
once it's processed (e.g., by functions like `ip_rcv()`), it will be directly mapped to the `skb` structure.
It is speculated that both IPv4 Differentiated Services Code Point (DSCP) and 802.1Q VLAN Priority Code Point (PCP) will be mapped to `skb->priority`.
