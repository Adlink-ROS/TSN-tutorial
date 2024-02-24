### Kata Container

Kata seems to support TSN, so we tried to use Kata, and failed.

**Doesn't support Ubuntu**

In short, Kata has moved to version 2.x and discontinued maintenance for version 1.x.
However, only version 1.x has an installation package for Ubuntu.
I tried *ubuntu-installation-guide* and *kata-manager*
All the content in links required for installing packages of version 1.x have been removed
(following the official installation steps doesn't provide any files to download).

**Has hardware requirement**

Kata Containers requires nested virtualization or bare metal.

### Install on Fedora38

installation guide:
https://github.com/kata-containers/kata-containers/blob/main/docs/how-to/containerd-kata.md#install-cri-tools

To install Kata, we have to install:
- Kata Containers
- Containerd with CRI plugin
    -  Install Containerd
    -  Install Kubeadm, Kubelet and Kubectl (remember to open the required ports!)
- CNI plugins
- cri-tools
