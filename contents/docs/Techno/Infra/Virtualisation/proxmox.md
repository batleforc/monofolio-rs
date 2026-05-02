---
date: 2026-05-12T22:49:00Z
title: "Proxmox"
description: "Proxmox VE - Virtualisation Platform"
spec:
  blog: false
  project: false
  doc: true
mindmap:
  include: true
  maturity: advanced
links:
  - name: "Proxmox VE"
    url: "https://www.proxmox.com/en/proxmox-ve"
tags:
  - "Virtualisation"
  - "Proxmox"
image: "icomoon#proxmox"
---

## Proxmox VE

Proxmox VE (Virtual Environment) is an open-source server virtualization platform. It allows you to run virtual machines and containers on a single physical server, providing a powerful and flexible solution for managing your virtual infrastructure. Proxmox VE supports both KVM for full virtualization and LXC for container-based virtualization, making it a versatile choice for various use cases.

I used Proxmox at first to run a few VM for testing purposes. After some time, i got further down the rabbit hole and started using it to orchestrate the creation of Talos node with the use of Cluster API. It's the [V1 of Weebo-SI](https://batleforc.github.io/weebo-si/1.Proxmox/architecture.html).
