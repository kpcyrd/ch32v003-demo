# ch32v003-demo

Simple hello world example code for the ch32v003 microcontroller
(ch32v003f4p6), using the [ch32-hal] crate. Based on the [ch32-hal-template]
repository.

[ch32-hal]: https://github.com/ch32-rs/ch32-hal
[ch32-hal-template]: https://github.com/ch32-rs/ch32-hal-template

## Compile

```
RUSTC_BOOTSTRAP=1 cargo build --release
```

## Flashing

The project has been setup to use [wlink] to flash the firmware (using a
WCH-LinkE).

**Note:** The usb-c port is only used for power supply and can't be used for
programming. It also commonly misses the 5.1 kΩ resistor so it's not detected
as a proper device by many usb-c chargers/powerbanks. You may need an usb-a to
usb-c cable, since those can't negotiate and give 5V unconditionally.

[wlink]: https://github.com/ch32-rs/wlink

```
------------------,     ,---------------
            3V3   | <-> | V
WCH-LinkE   GND   | <-> | G     ch32v003
            SWDIO | <-> | SWD
------------------´     `---------------
```

```
RUSTC_BOOTSTRAP=1 cargo run --release
```

## About `PD1`

I haven't tested this myself, but I read a few times this pin is the same one
as `SWD` used for flashing and debugging.

Using this pin in your programs may interfere with further
flashing/programming, although it's possible to [unbrick], you probably want to
leave this pin unused.

[unbrick]: https://baileytownsend.dev/articles/rust-on-the-10-cent-microcontroller#unbricking-your-chip

Copying over the relevant command for posterity:

```
wlink erase --method power-off --chip CH32V003
```

Again, I didn't test this myself.

## Useful references

- https://baileytownsend.dev/articles/rust-on-the-10-cent-microcontroller
- https://albertskog.se/ch32v-in-rust/
- https://github.com/ch32-rs/ch32-hal
- https://github.com/ch32-rs/wlink
- https://github.com/ch32-rs/ch32-hal-template
- https://github.com/wagiminator/CH32V003-GameConsole

## License

`MIT-0`
