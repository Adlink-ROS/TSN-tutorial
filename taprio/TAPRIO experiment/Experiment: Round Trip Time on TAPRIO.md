# Experiment: Round Trip Time on TAPRIO
## Experiment on 2023/08/30

**These are the process of designing the method to calculate RTT involved adding timestamps to the packets to measure the RTT of the packets.**

### Caculate round trip time of packets

payload size: 12B 
period: 200us


I calculate the 'time difference between sending and receiving the next packet.'
However, this doesn't account for issues like packet loss or excessive delay.

#### case1 (default)
P1:
Average time: 55.928µs
Minimum time: 10.4µs
Maximum time: 209.801µs

P2:
Average time: 55.581µs
Minimum time: 21.8µs
Maximum time: 134.481µs

P3:
Average time: 53.552µs
Minimum time: 5.12µs
Maximum time: 1.150364ms

The numbers don't seem quite right.
Change it to recording the time of sending and receiving, and then subtracting them in order.

P1:
Average time: 62.348µs
Minimum time: 41.2µs
Maximum time: 295.321µs


P2:
Average time: 70.064µs
Minimum time: 39.64µs
Maximum time: 273.681µs

P3:
Average time: 65.643µs
Minimum time: 44.161µs
Maximum time: 354.121µs

都是從一百多逐漸下降到50左右 偶爾會出現極大數字

**Add timestamps to packets**
P1:
Average time: 82.637µs
Minimum time: 51.751µs
Maximum time: 1.650689ms


P2:
Average time: 74.126µs
Minimum time: 52.645µs
Maximum time: 1.730556ms

P3:
Average time: 76.712µs
Minimum time: 53.017µs
Maximum time: 485.041µs


### case2
Change status every 600us
```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 4 \
                     map 0 1 2 3 0 1 2 3 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@0 1@0 1@0\
                     base-time 200 \
                     sched-entry S 03 600000 \
                     sched-entry S 05 600000 \
                     sched-entry S 09 600000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI

```
The delay has hardly changed; the interval is too large.

We keep encountering 'Error: Connection reset by peer (OS error 104).


**The transfer rate of the process kept changing, and we attempted to isolate this issue. After two weeks of unsuccessful troubleshooting, we eventually resolved it by directly replacing two servers.**
### One process at a time
Only one process is running at a time to ensure that the fluctuating transfer rate is not caused by competing processes.
```
Average time: 68.97µs
Minimum time: 57.069µs
Maximum time: 166.001µs
```
Not working.

### taskset
**Excluding the possibility of it being caused by CPU context switching.**
task -c 0
even worse
```
Average time: 101.152µs
Minimum time: 62.675µs
Maximum time: 297.825µs
```
task -c 1
```
Average time: 93.982µs
Minimum time: 58.43µs
Maximum time: 357.045µs
```
task -c 3
```
Average time: 69.43µs
Minimum time: 58.663µs
Maximum time: 159.281µs
```
It's possible that the first two cores have other processes (confirmed with htop that cores 0 and 1 are busier).
However, even when using idle cores, the issue of fluctuating transfer rates persists.

### 2 open at a time
When two processes with lower priority are running concurrently, the maximum time is significantly higher, indicating a competitive situation.
```
Average time: 72.215µs
Minimum time: 59.371µs
Maximum time: 590.486µs
```
```
Average time: 69.15µs
Minimum time: 59.612µs
Maximum time: 182.886µs
```

### Accept the unexpected result
To mitigate the fluctuations, we increased the time slot interval to the extent that these fluctuations can be ignored.
The transmission period is 200us, and the sched-entry is in ns.
```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 4 \
                     map 0 1 2 3 0 1 2 3 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@0 1@0 1@0\
                     base-time 200 \
                     sched-entry S 03 1600000 \
                     sched-entry S 05 1600000 \
                     sched-entry S 09 1600000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
```
p1
```
Average time: 74.634µs
Minimum time: 57.235µs
Maximum time: 170.689µs
```
p2
```
Average time: 72.265µs
Minimum time: 58.834µs
Maximum time: 408.451µs
```
p3
```
Average time: 99.642µs
Minimum time: 53.076µs
Maximum time: 1.900506ms
```
This isn't right because each priority should have to wait for 3.2ms before it gets its turn.


Switched to two new servers with a direct connection.

After transitioning to the new host, it appears that iperf3 is quite stable.
However, it's essential to note that iperf measures the traffic within a 1-second window,
so it may not always provide a reliable reference.
When observing the round-trip time, it remains unstable and, in some cases, even worse than before.

Settings:
- Neither end has taprio configuration.
- Payload size: 64 bytes
- Sending interval: 200 microseconds

```
Average time: 199.052µs
Minimum time: 48.815µs
Maximum time: 31.417708ms
```

This result remains consistent over multiple runs.

Adding "sudo nice -n -20":

```
Average time: 591.642µs
Minimum time: 65.247µs
Maximum time: 29.349799ms
```

However, there is no significant improvement in stability.

At times, there is a period of stability (after many iterations):

```
Average time: 60.415µs
Minimum time: 47.5µs
Maximum time: 155.883µs
```

Changing the sending interval to 200 milliseconds for a 50-second run:

```
Average time: 176.56µs
Minimum time: 113.923µs
Maximum time: 255.121µs
```

For a 200-second run:

```
Average time: 175.548µs
Minimum time: 91.797µs
Maximum time: 335.581µs
```

These results are similar to using ping (rtt min/avg/max/mdev = 0.118/0.224/0.868/0.056 ms).
The extreme fluctuations have disappeared, suggesting that the previous sending rate was too high.
However, it is not logical that reducing the number of packets leads to an increase in average round-trip time.

Using a period of 1ms as a compromise:

```
Average time: 66.259µs
Minimum time: 49.64µs
Maximum time: 272.893µs
```

And:

```
Average time: 99.947µs
Minimum time: 54.235µs
Maximum time: 336.173µs
```

Similar to the results obtained using ping (rtt min/avg/max/mdev = 0.118/0.224/0.868/0.056 ms), the extreme cases have disappeared (there are no millisecond-level values).
This suggests that the original packet transmission rate may have been too dense.
However, it's not reasonable that reducing the number of packets results in an increase in the average round-trip time.

As a compromise, with a period of 1ms:


```
Average time: 66.259µs
Minimum time: 49.64µs
Maximum time: 272.893µs
```
```
Average time: 66.259µs
Minimum time: 49.64µs
Maximum time: 272.893µs
```
When using ping (default), the round-trip times are consistently around 0.2ms, approximately 200us.
```
64 bytes from 192.168.1.2: icmp_seq=1 ttl=64 time=0.226 ms
64 bytes from 192.168.1.2: icmp_seq=2 ttl=64 time=0.209 ms
64 bytes from 192.168.1.2: icmp_seq=3 ttl=64 time=0.206 ms
64 bytes from 192.168.1.2: icmp_seq=4 ttl=64 time=0.202 ms
64 bytes from 192.168.1.2: icmp_seq=5 ttl=64 time=0.213 ms
64 bytes from 192.168.1.2: icmp_seq=6 ttl=64 time=0.201 ms
64 bytes from 192.168.1.2: icmp_seq=7 ttl=64 time=0.243 ms
64 bytes from 192.168.1.2: icmp_seq=8 ttl=64 time=0.205 ms
```

Adding "Packet Spacing Time" at the server end involves subtracting the timestamps of two consecutive packets.
Here are the settings:

Neither end has taprio configuration.
Payload size: 64 bytes
Sending interval: 200 microseconds
```
Average time: 203.2µs
Minimum time: 121.67µs
Maximum time: 242.683µs
```

Adding "Receive Time" at the server end involved an attempt to serialize Instant data into packets.
However, the underlying infrastructure for this type of operation is not implemented.
Several methods found on the internet are based on system clocks and are not suitable for use between two separate computers.

#### TAPRIO
```
sudo tc qdisc replace dev enp3s0 parent root handle 100 taprio \
     num_tc 3 \
     map 0 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 \
     queues 1@0 1@1 1@2 \
     base-time 200 \
     sched-entry S 01 300000 \
     sched-entry S 03 300000 \
     sched-entry S 05 800000 \
     flags 0x1 \
     txtime-delay 200000 \
     clockid CLOCK_TAI
```
*All in multiples of 100us*
*p1 should be blocked for 1100us*
period=200ms
priority=1
payload_size=64B

The results do not appear to be significantly affected.
```
Average time: 187.496µs
Minimum time: 153.184µs
Maximum time: 218.958µs
```
Disabling tc1 confirmed that it doesn't impact the results.
This could be due to the packet sending interval being too long.
```
period=1ms
priority=1
payload_size=64B
```
Average time: 100.456µs
Minimum time: 54.872µs
Maximum time: 320.681µs
```
Switching to a setting that is a multiple of 1ms, and larger periods:
```
sudo tc qdisc replace dev enp3s0 parent root handle 100 taprio \
     num_tc 3 \
     map 0 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 \
     queues 1@0 1@1 1@2 \
     base-time 200 \
     sched-entry S 01 3000000 \
     sched-entry S 03 3000000 \
     sched-entry S 05 8000000 \
     flags 0x1 \
     txtime-delay 200000 \
     clockid CLOCK_TAI
```
*All in multiples of 1ms*
*p1 should be blocked for 11ms*
period=1ms
priority=1
payload_size=64B
*unexpectedly faster*
```
Average time: 64.732µs
Minimum time: 51.079µs
Maximum time: 272.413µs
```
Trying priority set to 2 yielded similar results.
Additionally, attempting ping (equivalent to priority=0) showed the same results, but ping behaved normally.
