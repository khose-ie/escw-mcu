# Embedded Software C Wrapper of MCU.

A trait interface for doing the OOP wrapper for the C language code of MCU firmware libraries.
It could not work independentlly,  it should work with an implementation crate for all trains in this crate.

## Features

The feature of this crate is to divide the code by the different peripherals of a MCU.

| Features | Description                                       |
| :------: | ------------------------------------------------- |
|   `io`   | Enable the GPIO related trait and definations.    |
|  `uart`  | Enable the U(S)ART related trait and definations. |

By default, all features are not be set as the default feature, because every peripheral may not be used in a project.

So, please open the related features if you use this peripheral in your project.

```toml
[dependencies]
escw-mcu = { version = "0.0.1", feature = ["io", "uart"] }
```

