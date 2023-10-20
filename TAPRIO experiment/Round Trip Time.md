# Experiment on 2023/08/30
**These are the process of designing the method to calculate RTT involved adding timestamps to the packets to measure the RTT of the packets.**

## Caculate round trip time of packets

payload size: 12B 
period: 200us


I calculate the 'time difference between sending and receiving the next packet.' However, this doesn't account for issues like packet loss or excessive delay.
## case1 (default)
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


## case2
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

# Experiment on 2023/09/15
**The transfer rate of the process kept changing, and we attempted to isolate this issue. After two weeks of unsuccessful troubleshooting, we eventually resolved it by directly replacing two servers.**
# 202309015meeting
## One process at a time
Only one process is running at a time to ensure that the fluctuating transfer rate is not caused by competing processes.
```
Average time: 68.97µs
Minimum time: 57.069µs
Maximum time: 166.001µs
```
Not working.

## taskset
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
It's possible that the first two cores have other processes (confirmed with htop that cores 0 and 1 are busier). However, even when using idle cores, the issue of fluctuating transfer rates persists.

## 2 open at a time
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

## Accept the unexpected result
To mitigate the fluctuations, we increased the time slot interval to the extent that these fluctuations can be ignored. The transmission period is 200us, and the sched-entry is in ns.
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

