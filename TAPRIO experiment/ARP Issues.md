# ARP issues
## socket-priority experiment on 2023/08/11
### TAPRIO
```
map 2 2 1 0 2 2 2 2 2 2 2 2 2 2 2 2
SO_PRIORITY value 3 maps to TC 0, while value 2 maps to TC 1.

queues 1@0 1@1 2@2 
"queues 0 1 2 2" is a positional argument, meaning that TC 0 maps to queue 0, TC 1 maps to queue 1 and TC 2 maps to queues 2 and 3. 
Everything else maps to the other (best-effort) traffic classes;
```
(Both placed in ~/socket-priority)
ROS:
enp1s0 192.168.7.1
vlan1@enp1s0 192.168.1.1
```
./target/release/server --listen-addr=192.168.1.1:55555 --priority=6

（receiver）
```
Billie:
enP4p4s0 192.168.7.2
vlan1@enP4p4s0 192.168.1.2
```
./target/release/client --connect-addr 192.168.1.1:55555 --priority=6 --payload-size=64

（sender）
```
TAPRIO is for egress, so it should be configured on Billie.

```
chmod +x parallel.sh
```

Experiment:
Set an extremely large value at priority 2
./target/release/client --connect-addr 192.168.1.1:55552 --priority=2 --payload-size=25600
It's approximately 5.2 Gbits/s (we found that it should is unusual after weeks later).

### WithoutTAPRIO

```
./target/release/client --connect-addr 192.168.1.1:55556 --priority=6 --payload-size=64 > p6.txt 2>&1 &
./target/release/client --connect-addr 192.168.1.1:55552 --priority=2 --payload-size=12800 > p2.txt 2>&1 &
```
ROS(Server):
```
p6.txt
0.597 Gbits
0.581 Gbits
0.587 Gbits
0.585 Gbits
0.586 Gbits
0.588 Gbits
0.588 Gbits
0.587 Gbits
0.587 Gbits
0.587 Gbits

p2.txt
5.116 Gbits
5.059 Gbits
5.041 Gbits
5.045 Gbits
5.032 Gbits
5.171 Gbits
5.375 Gbits
5.216 Gbits
5.251 Gbits
5.230 Gbits
```
### With TAPRIO
```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 3 \
                     map 0 0 0 0 0 0 2 0 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@1 1@2 \
                     base-time 200 \
                     sched-entry S 00 000000 \
                     sched-entry S 02 300000 \
                     sched-entry S 04 800000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI


# This configuration maps p2 to TC0, q0, but no traffic is being transmitted successfully from q0.
# p6 is mapped to TC2, q2.
```
The result is that no traffic is received, and the sender encounters the error "No route to host (os error 113)."
```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 3 \
                     map 0 1 1 0 0 0 2 0 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@1 1@2 \
                     base-time 200 \
                     sched-entry S 07 000000 \
                     sched-entry S 02 300000 \
                     sched-entry S 04 800000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI



# This configuration maps p2 to TC1, q1, and q1 should be capable of transmitting.

# "00" means all traffic classes are turned off, "02" indicates that q1 is enabled, and "04" indicates that q2 is enabled.

# The unit of time is microseconds (us).
```
Error: No route to host (os error 113)

```
sudo ip link show dev vlan1
5: vlan1@enP5p1s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP mode DEFAULT group default qlen 1000
    link/ether 08:26:97:f7:49:c5 brd ff:ff:ff:ff:ff:ff
```
```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 3 \
                     map 0 1 1 0 0 0 2 0 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@1 1@2 \
                     base-time 0 \
                     sched-entry S 05 500000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI

```
```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 3 \
                     map 0 1 1 0 0 0 2 0 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@1 1@2 \
                     base-time 0 \
                     sched-entry S 07 500000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
# success
```
02, 05, 80, 40, 01 failed
ff, 0f, 03 successed
03=>011

```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 3 \
                     map 0 1 1 0 0 0 2 0 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@1 1@2 \
                     base-time 200 \
                     sched-entry S 01 000000 \
                     sched-entry S 02 300000 \
                     sched-entry S 04 800000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
```
failed

Two kinds of error, 
Error: No route to host (os error 113)
Error: Connection timed out (os error 110)

```
tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 3 \
                     map 0 1 1 0 0 0 2 0 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@1 1@2 \
                     base-time 200 \
                     sched-entry S 01 800000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI

```
6 failed，other successed
```
sched-entry S 00 800000
```
all failed


### Transmitting 7 Different Priorities Simultaneously
p0, p1, p2, and p3 correspond to TC0, TC1, TC2, and TC3, respectively.
p6 corresponds to TC2.
The remaining priorities are directed to TC0.
TC0, TC1, TC2, and TC3 map to q0, q1, q2, and q3, respectively.
#### Settings
```
tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 4 \
                     map 0 1 2 3 0 0 2 0 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@1 1@2 1@3\
                     base-time 200 \
                     sched-entry S 02 800000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
```


#### 00 <0000>:
none

#### 01 <0001>:
p0, p4, p5
（queue0)
#### 02 <0010>:
p1 (but there may have been an error previously?)
(queue1)
The issue disappeared during the second test.

#### 03 <0011>:
p0, p1, p4, p5
(queue0, 1)

#### 04 <0100>:
None

#### 05 <0101>:
p0, p2, p4, p5, p6
(queue0, 2)

#### 06 <0110>:
First time: 1 (but there may have been an error previously?)
Second time: None

#### 07 <0111>:
First time: p0, p1, p4, p5, p6
(queue0, 1)
(If queue 2 is enabled, p2 and p6 should transmit successfully)
First time: p0, p1, p2, p4, p5, p6

#### 08 <1000>:
None

#### 09 <1001>:
p0, p3, p4, p5
(queue0, 3)

#### 0a <1010>:
None

#### 0b <1011>:
p0, p1, p3, p4, p5
(queue0, 1, 3)

#### 0c <1100>:
None

#### 0d <1101>:
p0, p2, p3, p4, p5, p6
(queue0, 2, 3)

#### 0e <1110>:
None

#### 0f <1111>:
All

#### Current Observations
1. When the client modifies the qdisc rules, the server needs to be restarted (sometimes not restarting the program results in the inability to receive any data).
2. There are two different error messages:
   - Error: Connection timed out (os error 110)
   - Error: No route to host (os error 113)
3. Occasionally, retesting produces different results.

## socket-priority experiment on 2023/08/18
### TAPRIO
```
sudo ip link set dev vlan1 type vlan egress 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7
```
Changing the packet size to 12 bytes and the interval to 1600000.

#### 00 <0000>:
none

#### 01 <0001>:
p0

#### 02 <0010>:
First attempt: None 
```Error: No route to host (os error 113)```
Second attempt: Client-side no output, but the server-side received data. p0 and p1 both received data.
Third attempt: Both sides are working normally.
Fourth attempt：none 
```Error: No route to host (os error 113)```
Reciever gets" 60 418.234556963 ZyxelCom_f7:49:c9 → Broadcast    ARP 46 Who has 192.168.1.2? Tell 192.168.1.1"
But not limited to the sender running.

#### Error: Connection timed out (os error 110)
Focusing only on p1.
All four interfaces are monitored, and the same pattern is observed: ARP requests for 192.168.1.2.
such as
```
30 65.535682937 ZyxelCom_f7:49:c9 → Broadcast    ARP 64 Who has 192.168.1.2? Tell 192.168.1.1
```


#### Error: No route to host (os error 113)
Repeating the same scenario.
Only the client vlan1 has Broadcast, and later all four interfaces have similar traffic:
6 0.000215873 192.168.7.1 → 224.0.0.251 MDNS 84 Standard query 0x0000 PTR _digitalpaper._tcp.local, "QM" question


#### 03 <0011>:
p0, p1
Sender also sees:
1 0.000000000 ZyxelCom_f7:49:c5 → Broadcast ARP 42 Who has 192.168.1.1? Tell 192.168.1.2
But then it proceeds fine.

When the client is not producing output：
1 0.000000000 ZyxelCom_f7:49:c5 → Broadcast ARP 42 Who has 192.168.1.1? Tell 192.168.1.2
This issue is only observed when the client is running.

Retesting:

Sender doesn't capture anything (neither on the physical nor VLAN interfaces).
Another retest:

All four interfaces are monitored.
The client doesn't report errors or display anything.
All four interfaces have a small amount of traffic between 192.168.1.2 → 192.168.1.1 and 192.168.1.1 → 192.168.1.2.



# trace
## S 02
### Error: No route to host (os error 113)
```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
100.00    0.078211       39105         2         1 wait4
  0.00    0.000000           0         1           dup3
  0.00    0.000000           0         3         1 fcntl
  0.00    0.000000           0        11         2 ioctl
  0.00    0.000000           0         7         2 faccessat
  0.00    0.000000           0         9           openat
  0.00    0.000000           0         9           close
  0.00    0.000000           0         4           lseek
  0.00    0.000000           0         9           read
  0.00    0.000000           0        21         4 newfstatat
  0.00    0.000000           0         8           fstat
  0.00    0.000000           0        16           rt_sigaction
  0.00    0.000000           0        11           rt_sigprocmask
  0.00    0.000000           0         1           rt_sigreturn
  0.00    0.000000           0         1           getpgid
  0.00    0.000000           0         1           uname
  0.00    0.000000           0         2           getpid
  0.00    0.000000           0         1           getppid
  0.00    0.000000           0         7           getuid
  0.00    0.000000           0         7           geteuid
  0.00    0.000000           0         7           getgid
  0.00    0.000000           0         7           getegid
  0.00    0.000000           0         1           sysinfo
  0.00    0.000000           0         4           brk
  0.00    0.000000           0         1           munmap
  0.00    0.000000           0         1           clone
  0.00    0.000000           0         1           execve
  0.00    0.000000           0        12           mmap
  0.00    0.000000           0         8           mprotect
  0.00    0.000000           0         2           prlimit64
------ ----------- ----------- --------- --------- ----------------
100.00    0.078211                   175        10 total

```
### Error: Connection timed out (os error 110)
```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
100.00    0.071706       35853         2         1 wait4
  0.00    0.000000           0         1           dup3
  0.00    0.000000           0         3         1 fcntl
  0.00    0.000000           0        11         2 ioctl
  0.00    0.000000           0         7         2 faccessat
  0.00    0.000000           0         9           openat
  0.00    0.000000           0         9           close
  0.00    0.000000           0         4           lseek
  0.00    0.000000           0         9           read
  0.00    0.000000           0        21         4 newfstatat
  0.00    0.000000           0         8           fstat
  0.00    0.000000           0        16           rt_sigaction
  0.00    0.000000           0        11           rt_sigprocmask
  0.00    0.000000           0         1           rt_sigreturn
  0.00    0.000000           0         1           getpgid
  0.00    0.000000           0         1           uname
  0.00    0.000000           0         2           getpid
  0.00    0.000000           0         1           getppid
  0.00    0.000000           0         7           getuid
  0.00    0.000000           0         7           geteuid
  0.00    0.000000           0         7           getgid
  0.00    0.000000           0         7           getegid
  0.00    0.000000           0         1           sysinfo
  0.00    0.000000           0         4           brk
  0.00    0.000000           0         1           munmap
  0.00    0.000000           0         1           clone
  0.00    0.000000           0         1           execve
  0.00    0.000000           0        12           mmap
  0.00    0.000000           0         8           mprotect
  0.00    0.000000           0         2           prlimit64
------ ----------- ----------- --------- --------- ----------------
100.00    0.071706                   175        10 total
```

## S 03
### can pass p0 p1
```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
  0.00    0.000000           0         1           dup3
  0.00    0.000000           0         3         1 fcntl
  0.00    0.000000           0         3         2 ioctl
  0.00    0.000000           0         5         1 faccessat
  0.00    0.000000           0         8           openat
  0.00    0.000000           0         8           close
  0.00    0.000000           0         4           lseek
  0.00    0.000000           0         5           read
  0.00    0.000000           0        17         3 newfstatat
  0.00    0.000000           0         7           fstat
  0.00    0.000000           0        15           rt_sigaction
  0.00    0.000000           0         8           rt_sigprocmask
  0.00    0.000000           0         1           getpgid
  0.00    0.000000           0         1           uname
  0.00    0.000000           0         2           getpid
  0.00    0.000000           0         1           getppid
  0.00    0.000000           0         5           getuid
  0.00    0.000000           0         5           geteuid
  0.00    0.000000           0         5           getgid
  0.00    0.000000           0         5           getegid
  0.00    0.000000           0         1           sysinfo
  0.00    0.000000           0         3           brk
  0.00    0.000000           0         1           munmap
  0.00    0.000000           0         1           clone
  0.00    0.000000           0         1           execve
  0.00    0.000000           0        12           mmap
  0.00    0.000000           0         8           mprotect
  0.00    0.000000           0         1         1 wait4
  0.00    0.000000           0         2           prlimit64
------ ----------- ----------- --------- --------- ----------------
100.00    0.000000                   139         8 total
```
Three types of errors are all the same.

Reboot: The qdisc will be cleared and become ineffective.

## Only run the command
```
 strace -c ./target/release/client --connect-addr 192.168.1.1:55551 --priority=1 --payload-size=12
 ```
### S02
#### Error: No route to host (os error 113)
```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
  0.00    0.000000           0         1         1 faccessat
  0.00    0.000000           0         7           openat
  0.00    0.000000           0         8           close
  0.00    0.000000           0         9           read
  0.00    0.000000           0         6           write
  0.00    0.000000           0         1           ppoll
  0.00    0.000000           0         7           fstat
  0.00    0.000000           0         1           set_tid_address
  0.00    0.000000           0         1           set_robust_list
  0.00    0.000000           0         1           sched_getaffinity
  0.00    0.000000           0         3           sigaltstack
  0.00    0.000000           0         7           rt_sigaction
  0.00    0.000000           0         1           rt_sigprocmask
  0.00    0.000000           0         1           socket
  0.00    0.000000           0         1         1 connect
  0.00    0.000000           0         3           brk
  0.00    0.000000           0         2           munmap
  0.00    0.000000           0         1           execve
  0.00    0.000000           0        16           mmap
  0.00    0.000000           0        13           mprotect
  0.00    0.000000           0         2           prlimit64
------ ----------- ----------- --------- --------- ----------------
100.00    0.000000                    92         2 total
```



#### Error: Connection timed out (os error 110)
```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
 22.99    0.000303         303         1           execve
 11.91    0.000157           9        16           mmap
 10.32    0.000136          10        13           mprotect
  8.80    0.000116          12         9           read
  8.50    0.000112          16         7           openat
  6.68    0.000088          88         1         1 connect
  5.39    0.000071           8         8           close
  5.01    0.000066          33         2           munmap
  4.70    0.000062          10         6           write
  3.41    0.000045           6         7           fstat
  3.19    0.000042           6         7           rt_sigaction
  1.52    0.000020          20         1           socket
  1.52    0.000020           6         3           brk
  1.29    0.000017           5         3           sigaltstack
  1.14    0.000015          15         1         1 faccessat
  0.99    0.000013          13         1           ppoll
  0.91    0.000012           6         2           prlimit64
  0.46    0.000006           6         1           set_tid_address
  0.46    0.000006           6         1           sched_getaffinity
  0.46    0.000006           6         1           rt_sigprocmask
  0.38    0.000005           5         1           set_robust_list
------ ----------- ----------- --------- --------- ----------------
100.00    0.001318                    92         2 total


```
### S03
```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
100.00    0.160262           4     35040           sendto
  0.00    0.000000           0         1         1 faccessat
  0.00    0.000000           0         7           openat
  0.00    0.000000           0         7           close
  0.00    0.000000           0         9           read
  0.00    0.000000           0         1           ppoll
  0.00    0.000000           0         7           fstat
  0.00    0.000000           0         1           set_tid_address
  0.00    0.000000           0         1           set_robust_list
  0.00    0.000000           0         1           sched_getaffinity
  0.00    0.000000           0         2           sigaltstack
  0.00    0.000000           0         7           rt_sigaction
  0.00    0.000000           0         1           rt_sigprocmask
  0.00    0.000000           0         1           socket
  0.00    0.000000           0         1           connect
  0.00    0.000000           0         1           setsockopt
  0.00    0.000000           0         1           getsockopt
  0.00    0.000000           0         3           brk
  0.00    0.000000           0         1           munmap
  0.00    0.000000           0         1           execve
  0.00    0.000000           0        16           mmap
  0.00    0.000000           0        13           mprotect
  0.00    0.000000           0         2           prlimit64
------ ----------- ----------- --------- --------- ----------------
100.00    0.160262                 35125         1 total
```

## Only trace network related infomations
```
strace -e trace=network ./target/release/client --connect-addr 192.168.1.1:55551 --priority=1 --payload-size=12 2>&1
```
### S 02
```
socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC, IPPROTO_IP) = 3
connect(3, {sa_family=AF_INET, sin_port=htons(55551), sin_addr=inet_addr("192.168.1.1")}, 16) = -1 EHOSTUNREACH (No route to host)
Error: No route to host (os error 113)
+++ exited with 1 +++
```

### S 03
```
socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC, IPPROTO_IP) = 3
connect(3, {sa_family=AF_INET, sin_port=htons(55551), sin_addr=inet_addr("192.168.1.1")}, 16) = 0
setsockopt(3, SOL_SOCKET, SO_PRIORITY, [1], 4) = 0
getsockopt(3, SOL_SOCKET, SO_PRIORITY, [1], [4]) = 0
sendto(3, "\0\0\0\0\0\0\0\0\0\0\0\0", 12, MSG_NOSIGNAL, NULL, 0) = 12
# the last line repeats
```

## check route
### S 02
#### Error: Connection timed out (os error 110)
```
default via 10.88.15.254 dev enx00051ba483ac proto dhcp metric 100 
10.8.0.0/24 via 10.8.0.1 dev tun0 
10.8.0.1 dev tun0 proto kernel scope link src 10.8.0.252 
10.88.0.0/20 dev enx00051ba483ac proto kernel scope link src 10.88.14.227 metric 100 
172.17.0.0/16 dev docker0 proto kernel scope link src 172.17.0.1 linkdown 
192.168.1.0/24 dev vlan1 proto kernel scope link src 192.168.1.2 metric 400 
192.168.7.0/24 dev enP5p1s0 proto kernel scope link src 192.168.7.2 metric 101 

```
#### Error: No route to host (os error 113)
```
default via 10.88.15.254 dev enx00051ba483ac proto dhcp metric 100 
10.8.0.0/24 via 10.8.0.1 dev tun0 
10.8.0.1 dev tun0 proto kernel scope link src 10.8.0.252 
10.88.0.0/20 dev enx00051ba483ac proto kernel scope link src 10.88.14.227 metric 100 
172.17.0.0/16 dev docker0 proto kernel scope link src 172.17.0.1 linkdown 
192.168.1.0/24 dev vlan1 proto kernel scope link src 192.168.1.2 metric 400 
192.168.7.0/24 dev enP5p1s0 proto kernel scope link src 192.168.7.2 metric 101 
```
### S 03
```
default via 10.88.15.254 dev enx00051ba483ac proto dhcp metric 100 
10.8.0.0/24 via 10.8.0.1 dev tun0 
10.8.0.1 dev tun0 proto kernel scope link src 10.8.0.252 
10.88.0.0/20 dev enx00051ba483ac proto kernel scope link src 10.88.14.227 metric 100 
172.17.0.0/16 dev docker0 proto kernel scope link src 172.17.0.1 linkdown 
192.168.1.0/24 dev vlan1 proto kernel scope link src 192.168.1.2 metric 400 
192.168.7.0/24 dev enP5p1s0 proto kernel scope link src 192.168.7.2 metric 101 
```
Those three are same, route table should be fine
netstat -r also have the same result

## traceroute
### S 02
#### Error: Connection timed out (os error 110)

#### Error: No route to host (os error 113)
```
traceroute to 192.168.1.1 (192.168.1.1), 30 hops max, 60 byte packets
 1  arm-billie (192.168.1.2)  3078.473 ms !H  3078.436 ms !H  3078.428 ms !H
 ```

### S 03
```
traceroute to 192.168.1.1 (192.168.1.1), 30 hops max, 60 byte packets
 1  192.168.1.1 (192.168.1.1)  0.174 ms  0.207 ms  0.133 ms

```
看不出個所以然


## queues的設定
```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 4 \
                     map 0 1 2 3 0 1 2 3 0 0 0 0 0 0 0 0 \
                     queues 2@0 1@1 1@2\
                     base-time 200 \
                     sched-entry S 02 1600000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
# Error: Invalid queue in traffic class to queue mapping.
```
```
for (i = 0; i < qopt->num_tc; i++) {
		unsigned int last = qopt->offset[i] + qopt->count[i];

		/* Verify the queue count is in tx range being equal to the
		 * real_num_tx_queues indicates the last queue is in use.
		 */
		if (qopt->offset[i] >= dev->num_tx_queues ||
		    !qopt->count[i] ||
		    last > dev->real_num_tx_queues) {
			NL_SET_ERR_MSG(extack, "Invalid queue in traffic class to queue mapping");
			return -EINVAL;
		}

		if (TXTIME_ASSIST_IS_ENABLED(taprio_flags))
			continue;

		/* Verify that the offset and counts do not overlap */
		for (j = i + 1; j < qopt->num_tc; j++) {
			if (last > qopt->offset[j]) {
				NL_SET_ERR_MSG(extack, "Detected overlap in the traffic class to queue mapping");
				return -EINVAL;
			}
		}
	}
```
```
tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 4 \
                     map 0 1 2 3 0 1 2 3 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@1 2@2\
                     base-time 200 \
                     sched-entry S 02 1600000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
# Error: Invalid queue in traffic class to queue mapping.
```
When there are only three items in queues, num_tc can only be 3. If num_tc is set to 4, it will trigger (!qopt->count[i]).

```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 3 \
                     map 0 1 2 0 0 1 2 0 0 0 0 0 0 0 0 0 \
                     queues 2@0 1@1 1@2\
                     base-time 200 \
                     sched-entry S 02 1600000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
# This is fine
```
```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 4 \
                     map 0 1 2 3 0 1 2 3 0 0 0 0 0 0 0 0 \
                     queues 2@0 1@1 1@2 1@3\
                     base-time 200 \
                     sched-entry S 02 1600000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
# Also fine
```

It's worth noting that this line:
```
unsigned int last = qopt->offset[i] + qopt->count[i];
```
implies whether one traffic class (tc) can correspond to multiple queues.


### Wierd
```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 4 \
                     map 0 1 2 3 0 1 2 3 0 0 0 0 0 0 0 0 \
                     queues 2@0 1@1 1@2 1@3\
                     base-time 200 \
                     sched-entry S 02 1600000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
# tc0->q0, q1
# tc1->q1
# tc2->q2
# tc3->q3
```
```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 4 \
                     map 0 1 2 3 0 1 2 3 0 0 0 0 0 0 0 0 \
                     queues 2@0 1@2 1@3 1@4\
                     base-time 200 \
                     sched-entry S 02 1600000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
# tc0->q0, q1
# tc1->q2
# tc2->q3
# tc3->q4 (???) we should on have 4 queues
```
### Wrong interface？？？
```
sudo ethtool -l enP4p4s0
Channel parameters for enP4p4s0:
Pre-set maximums:
RX:		0
TX:		0
Other:		1
Combined:	4
Current hardware settings:
RX:		0
TX:		0
Other:		1
Combined:	4

```
```
 sudo ethtool -l enP5p1s0
Channel parameters for enP5p1s0:
Cannot get device channel parameters
: Operation not supported
```
However, based on the previous experience, a qdisc command cannot be used when there are no multiple queues like this.
```
2: enP5p1s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc taprio state UP group default qlen 1000
    link/ether 08:26:97:f7:49:c5 brd ff:ff:ff:ff:ff:ff
    inet 192.168.7.2/24 brd 192.168.7.255 scope global noprefixroute enP5p1s0
       valid_lft forever preferred_lft forever
    inet6 fe80::6624:a8be:db8f:6f8f/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever
```
It has the same MAC address as the one with the cable plugged in.




## Checking if it's a queue or a traffic class issue
### S 02

```
sudo tc qdisc replace dev enP5p1s0 parent root handle 100 taprio \
                     num_tc 4 \
                     map 0 1 2 3 0 1 2 3 0 0 0 0 0 0 0 0 \
                     queues 1@0 1@0 1@0 1@0 \
                     base-time 200 \
                     sched-entry S 02 1600000 \
                     flags 0x1 \
                     txtime-delay 200000 \
                     clockid CLOCK_TAI
```

All are placed in q0.

#### Error: No route to host (OS error 113)

So, it appears to be a traffic class (tc) issue.

