---
date: 2026-05-12T22:49:00Z
title: "KubeVirt"
description: "KubeVirt - Virtualisation on Kubernetes"
spec:
  blog: false
  project: false
  doc: true
mindmap:
  include: true
  maturity: advanced
links:
  - name: "KubeVirt"
    url: "https://kubevirt.io/"
tags:
  - "Virtualisation"
  - "KubeVirt"
image: "icomoon#kubernetes"
---

## KubeVirt

KubeVirt is easily my favourite virtualisation engine. It allows you, following the same principles as Kubernetes, to run virtual machines inside of your kubernetes cluster. It also allows you to follow the same GitOps principles that you can apply to your kubernetes cluster.

I use KubeVirt to run different kind of VM and like Proxmox, i use it to orchestrate the creation of Talos node with the use of Cluster API. It's the [V2 of Weebo-SI](https://batleforc.github.io/weebo-si/1.Kubevirt/architecture.html).
