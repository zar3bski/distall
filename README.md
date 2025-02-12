# dist-urb

General purpose CLAP and VST3 distortion

## Current status and roadmap

`dist-urb` currently only implements a **Soft clipping** distortion with no filter as a POC, testing the [nih-plug](https://github.com/robbert-vdh/nih-plug/tree/master) it is based on. Future developments include

* low pass filter
* DRY / WET signal separation
* asymmetric non linear functions explorations
* non linearity with memory
* aliasing suppression by oversampling
* [Vizia](https://github.com/vizia/vizia) based UI


## Building

After installing [Rust](https://rustup.rs/), you can compile DistUrb as follows:

```shell
cargo xtask bundle dist-urb --release
```
