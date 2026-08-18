![Plugin Icon](assets/icon.png)

# OpenDeck Mirabox 293V3 Plugin

An unofficial OpenDeck plugin for the Mirabox 293V3 stream deck.

This is a fork of [4ndv/opendeck-akp153](https://github.com/4ndv/opendeck-akp153), stripped down to support only the Mirabox 293V3.

## OpenDeck version

Requires OpenDeck 2.5.0 or newer

## Supported devices

- Mirabox 293V3 (6603:1005, 6603:1006)

## Installation

1. Download an archive from [releases](https://github.com/skorokithakis/opendeck-akp153/releases)
2. In OpenDeck: Plugins -> Install from file
3. Linux: Download [udev rules](./40-opendeck-mirabox-293v3.rules) and install them by copying into `/etc/udev/rules.d/` and running `sudo udevadm control --reload-rules`
4. Unplug and plug again the device, restart OpenDeck

## Building

### Prerequisites

You'll need:

- A Linux OS of some sort
- Rust 1.87 and up
- [just](https://just.systems)

### Building a release package

```sh
$ just package
```
