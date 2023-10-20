# SETTINGS

## Background Knowledge
## Hardware
The number of queues that can be used on TAPRIO depends on the number of queues available on the hardware. To check the number of queues available on a network card, where "ethx" is the name of the NIC, use the following command:
```
ethtool -l ethx
```
## set LINUX priority by VLAN
### Why VLAN

In Linux, the implementation of qdisc is based on reading a data structure called the Socket Buffer (sk_buff). The system reads the priority value from skb->priority. This means that regardless of the type of packet being transmitted, once it's processed (e.g., by functions like ip_rcv()), it will be directly mapped to the skb structure. It is speculated that both IPv4 Differentiated Services Code Point (DSCP) and 802.1Q VLAN Priority Code Point (PCP) will be mapped to skb->priority.

It's important to note that reading priority does not necessarily require the use of VLAN, but 802.1Q uses VLAN for this purpose. Therefore, we configure the relationship between VLAN egress PCP and the SO_PRIORITY value.
```
sudo ip link set dev vlan1 type vlan egress 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7
```
![](https://hackmd.io/_uploads/BkfAVtLin.jpg)


Set the priority of the sender to 6,
Packets captured will show PCP=6:
```
18:06:30.891525 08:26:97:f7:49:c5 (oui Unknown) > 08:26:97:f7:49:c9 (oui Unknown), ethertype 802.1Q (0x8100), length 2966: vlan 1, p 6, ethertype IPv4 (0x0800), (tos 0x0, ttl 64, id 23093, offset 0, flags [DF], proto TCP (6), length 2948)
    192.168.1.2.36196 > ros-RSK.55555: Flags [P.], cksum 0x8eca (incorrect -> 0x27f2), seq 76361265:76364161, ack 0, win 502, options [nop,nop,TS val 3959431146 ecr 1510782482], length 2896

```
(default value of pcp is 0)

### Experimental settings
Server and client are linked directly/through a switch by a 10Gbps wire(?)


#### Server
interface: ens1
VLAN on ens1: 

#### Client
interface: enp3s0
## TAPRIO
defines how Linux networking stack priorities map into traffic classes and how traffic classes map into hardware queues

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

num_tc: number of traffic class
map: The relationship between Linux priority and traffic class.


## map LINUX priority
sudo ip link set dev vlan1 type vlan egress 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7


#### Other issue on TAPRIO

The official documentation states that the parameters for the queue are specified as "count@offset" and should not overlap. 
**Queue ranges for each traffic classes cannot overlap and must be a contiguous range of queues.**
There are two modes available,0x1, 0x2. Enabling the execution of the Task Admission Control (TAS) functionality either in software or hardware.

In one of the examples provided in the documentation, there is a configuration where flows with different priorities are directed to the same queue and executed in software. 
```
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

If this configuration is using mode 0x2, it is indeed possible for them to share the same queue.

