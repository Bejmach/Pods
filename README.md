# Pods
> Do you think games are silly little things?

A lightweight **notification daemon** written in Rust.

---

## Overview

`pods` is a simple notification backend for Linux.  
It stores, retrieves, and streams notifications via CLI commands — perfect for integration with bars, widgets, or custom rices (e.g. [eww](https://elkowar.github.io/eww/)).

Unlike traditional D-Bus daemons, `pods` focuses on simplicity:
- Persistent storage (using sqlite)
- JSON output for easy parsing
- Live streaming mode (`pods listen`) #Yes it prints data each second
- Works perfectly with minimal setups or custom UIs

---

## Usage

### Start the daemon
```bash
pods enable
```
Runs the background daemon and registers on D-Bus
(so apps using notify-send can send notifications).

### Add a notification
```bash
pods add <app_name> <summary> <body> [--app-icon <icon_str>]
```
Example:
```bash
pods add "discord" "new message" "message"
```
Adds a new notification to the local database

### Get recent totifications
```bash
pods get [--since <seconds>]
```
Fetch recent notifications in JSON format
default duration is 10 seconds

```bash
pods get --since 60
[{"app": "music","title": "Now Playing","body": "Weight of the World – J'Nique Nicole"}]
```

### Listen for updates
```bash
pods listen [--since <seconds>]
```
Continuously outputs recent notifications each second
Great for use with reactive UIs like eww

Example:
```bash
pods listem --since 20
```

### REMOVE DOES NOT WORK!!!

### Clear all notifications
```bash
pods clear
```
Removes all notifications from local db

## Example integration with eww

```yuck
(deflisten notifs :initial "[]"
  `pods listen`)
```

## Instalation

Build from source
```bash
git clone git@github.com:Bejmach/Pods.git
cd pods
cargo install --path .
```

Or manual install
```bash
cargo build --release
sudo cp target/release/pods /usr/local/bin
```
