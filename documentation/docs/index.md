---
hide:
  - navigation
  - toc
---

# Welcome to UbiHome!

UbiHome is a single executable that allows you to integrate any device running an OS into your smart home. 
It is designed to be lightweight and easy to use - similar to ESPHome.

- [Execute a command on a device](./examples/display_notification/index.md) based on a trigger in home assistant. 
- [Monitor the status of a device](./examples/system_ressources/index.md) with a custom command. 
- Integrate all of your _one off python scripts^TM^_ without thinking about connectivity or setting up yet another service.

See the [getting started](getting_started/index.md) guide for installation instructions.

Explore the [examples](examples/index.md) to see how to use UbiHome.

<!-- x-release-please-start-version -->

```bash
pi@raspberrypi:~/ $ ubihome
UbiHome - 0.8.11

UbiHome is a system which allows you to integrate any device running an OS into your smart home.
https://github.com/UbiHome/UbiHome

Usage: ubihome [OPTIONS] <COMMAND>

Commands:
  install    Install UbiHome
  uninstall  Uninstall UbiHome
  run        Run UbiHome manually.
  help       Print this message or the help of the given subcommand(s)

Options:
  -c, --configuration <configuration_file>
          Optional configuration file. If not provided, the default configuration will be used. [default: config.yaml]
  -h, --help
          Print help
  -V, --version
          Print version
```
<!-- x-release-please-end -->

## Roadmap

- Monitor connected bluetooth devices and maybe even proxy them to home assistant.
- [ ] Auto installation
  - [x] Windows (https://github.com/mullvad/windows-service-rs) https://medium.com/@aleksej.gudkov/rust-windows-service-example-building-a-windows-service-in-rust-907be67d2287
  - [x] Linux Service
    - debian (https://github.com/kornelski/cargo-deb/blob/fc34c45fafc3904cadf652473ff7e9e0344c605c/systemd.md)
  - [ ] MacOS?
- [ ] Templates and Services
- [ ] Additional Components:
  - [ ] HTTP and Web Enpoint
  - [ ] BLE (https://github.com/deviceplug/btleplug)
  - [ ] Bluetooth Proxy (https://esphome.io/components/bluetooth_proxy.html)     https://docs.rs/bluer/latest/bluer/
- [ ] Online Sensor
- [ ] Custom compilation for modular builds and custom extensions.
- [ ] Homeassistant Native API
- [ ] CLI for automatic generation of executables
- [ ] Builder Component similar to ESP Home
- [ ] Self update (https://github.com/jaemk/self_update)
- [x] GPIO https://github.com/golemparts/rppal
- [ ] Kernel events (e.g. Keyboard press) https://docs.rs/evdev/latest/evdev/
- [ ] Music Assistant Media Player integration for Windows / Linux (https://docs.rs/souvlaki/latest/souvlaki/)

... Control USB Devices?

Rust clippy: 
https://github.com/rust-lang/rust-clippy


Add Badges? 
https://github.com/squidfunk/mkdocs-material/discussions/7137