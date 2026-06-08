# Lock Guard
Lock guard is a background process that shuts down a computer when its locked and unplugged. It was designed with hyprland in mind, but it should work on all desktop environments where the lock screen is a executable.

## Why?
Sometimes the need arises to leave your computer at your desk, locking it provides some safety but its not the best. A device with full disk encryption will always be safest while powered off. The scenario this is ment to protect against is something like this.

1. Your computer is at your desk, plugged in.
2. You lock your computer because you need to leave it temporarily.
3. A bad actor enters the space where your computer is and takes it.
4. They exploit the io. This could be something like connecting the device to a wired network and sniffing the traffic.

With this, when the laptop is disconnected from the wall it will turn off. This will cause the computer to return to a more secure state.

If you are on the go, and not connected to power then this program will do nothing.

## Installation

You can easily install with cargo

```
cargo install lock-guard
```

or 

```
cargo binstall lock-guard
```

## Configuration
Configuration is handled through the cli.

```bash
> lock-guard --help

Shut down your computer automaticly if a lock is running and its unplugged

Usage: lock-guard [OPTIONS]

Options:
  -b, --battery <BATTERY>
          The battery to read from. Must exist under /sys/class/power_supply/ and have status
          
          Setting this to DISABLED will cause the program to immidiatly exit.
          
          [default: BAT1]

  -l, --lock-processes <LOCK_PROCESSES>
          The processes to watch. This should be your lock process. E.g hyprlock
          
          [default: swaylock,hyprlock]

      --poll-rate <POLL_RATE>
          The time the program waits before it refreshes
          
          [default: 0.5]

  -d, --delay <DELAY>
          The delay between unplug and shutdown
          
          [default: 10]

  -h, --help
          Print help (see a summary with '-h')
```

## Battery selection
You can find your battery by running
```
ls /sys/class/power_supply/
```

Its usually named `BAT1`

Setting --battery to DISABLED will disable the program. This is here to support dotfile variable substitution. This is for when your dotfiles needs to work on both desktops and laptops.

### Desktop enviroment - Hyprland
> hyprland.conf
```txt
exec-once = lock-guard --poll-rate 0.5 -l hyprlock,swaylock --delay 5 --battery BAT1
```
