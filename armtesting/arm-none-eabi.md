# `{arm,thumb}*-none-eabi(hf)?`

**Tier: 2**
- [arm(eb)?v7r-none-eabi(hf)?](armv7r-none-eabi.md)
- armv7a-none-eabi
- thumbv6m-none-eabi
- thumbv7m-none-eabi
- thumbv7em-none-eabi(hf)?
- thumbv8m.base-none-eabi
- thumbv8m.main-none-eabi(hf)?

**Tier: 3**
- [{arm,thumb}v4t-none-eabi](armv4t-none-eabi.md)
- [{arm,thumb}v5te-none-eabi](armv5te-none-eabi.md)
- armv7a-none-eabihf
- [armv8r-none-eabihf](armv8r-none-eabihf.md)

Bare-metal target for 32-bit ARM CPUs.

If a target has a `*hf` variant, that variant uses the hardware floating-point
ABI and enables some minimum set of floating-point features based on the FPU(s)
available in that processor family. To use the FPU without the hard float ABI,
select a non-`hf` target and additionally pass a suitable `target-cpu` flag,
and take care to enable the FPU at startup.

## Target maintainers

The `thumb*` targets are maintained by the Rust Embedded Working Group's
[Cortex-M team](https://github.com/rust-embeded/wg#the-cortex-m-team).

## Requirements

These targets are cross-compiled and use static linking.

By default, the `lld` linker included with Rust will be used; however, you may
want to use the GNU linker instead. This can be obtained for Windows/Mac/Linux
from the [Arm Developer Website][arm-gnu-toolchain], or possibly from your OS's
package manager. To use it, add the following to your `.cargo/config.toml`:

```toml
[target.<your-target>]
linker = "arm-none-eabi-ld"
```

The GNU linker can also be used by specifying `arm-none-eabi-gcc` as the
linker. This is needed when using GCC's link time optimization.

[arm-gnu-toolchain]: https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain

These targets don't provide a linker script, so you'll need to bring your own
according to the specific device you are using. Pass
`-Clink-arg=-Tyour_script.ld` as a rustc argument to make the linker use
`your_script.ld` during linking. For Cortex-M targets, [`cortex-m-rt`] provides
a suitable linker script.

[`cortex-m-rt`]: https://github.com/rust-embedded/cortex-m-rt

Targets named `thumb*` instead of `arm*`
generate Thumb-mode code by default. M-profile processors (`thumbv*m*-*`
targets) only support Thumb-mode code.
For the `arm*` targets, Thumb-mode code generation can be enabled by using
`-C target-feature=+thumb-mode`. Using the unstable
`#![feature(arm_target_feature)]`, the attribute
`#[target_feature(enable = "thumb-mode")]` can be applied to individual
`unsafe` functions to cause those functions to be compiled to Thumb-mode code.

## Building Rust Programs

For the Tier 3 targets in this family, Rust does not ship pre-compiled
artifacts.

Use the `build-std` nightly cargo feature to build the `core` library. You
can pass this as a command line argument to cargo, or your `.cargo/config.toml`
file might include the following lines:

```toml
[unstable]
build-std = ["core"]
```

Most of `core` should work as expected, with the following notes:
* If the target is not `*hf`, then floating-point operations are emulated in
  software.
* Integer division is also emulated in software on some targets, depending on
  the CPU.
* Architectures prior to ARMv7 don't have atomic instructions.

`alloc` is also supported, as long as you provide your own global allocator.

Rust programs are output as ELF files.

## Testing

This is a cross-compiled target that you will need to emulate during testing.

The exact emulator that you'll need depends on the specific device you want to
run your code on.

## Cross-compilation toolchains and C code

The target supports C code compiled with the `arm-none-eabi` target triple and
a suitable `-march` or `-mcpu` flag.

`gcc` or `clang` can be used, but note that `gcc` uses `-fshort-enums` by
default for `arm-none*` targets, while `clang` does not. `rustc` matches the
`gcc` behavior, i.e., the size of a `#[repr(C)] enum` in Rust can be as little
as 1 byte, rather than 4, as they are on `arm-linux` targets.

## Floating point on Cortex-M processors


The `*hf` targets assume that the device has a hardware FPU and lowers all
floating point operations to hardware instructions.  Additionally, this target
uses the "hard" float ABI, where floating point values are passed to/from
subroutines via FPU registers.

To opt in to double precision hardware FPU support on Cortex-M7, use the
`-C target-cpu=cortex-m7` flags.

It is also possible to use the hardware FPU while using the soft float ABI, by
using the `thumbv7em-none-eabi` target and passing `-C target-cpu=cortex-m4`
(or `cortex-m7`). In this case the FPU must be enabled at startup, before
entering any function which takes floating-point arguments or uses any
floating-point variables. The [`cortex-m`] crate provides the
`SCB::enable_fpu()` convenience function to enable the FPU.

## Target flags

CPU-specific code generation and features can be enabled using the
`-C target-cpu` and `-C target-feature` flags for rustc. The following
flags are available for the `thumb*` targets:

### `thumbv6m-none-eabi`

This is suitable for Arm Cortex-M0, Arm Cortex-M0+ and Arm Cortex-M1 processors. It does not support atomic compare-and-swap.

There are no relevant target CPU options.

There are no relevant feature flags, and the FPU is not available.

### `thumbv7m-none-eabi`

This is suitable for Arm Cortex-M3 processor. It supports atomic compare-and-swap.

#### Arm Cortex-M3

The target CPU is `cortex-m3`.

There are no relevant feature flags, and the FPU is not available.

#### `thumbv7m-none-eabi` Options

The target CPU options may apply scheduling optimisations but there are no additional features in `cortex-m3` above the architecture baseline.

| CPU       | Target CPU  | Target Features |
| --------- | ----------- | --------------- |
| Cortex-M3 | `cortex-m3` | None            |

### `thumbv7em-none-eabi`

This is suitable for Arm Cortex-M4 and Arm Cortex-M7 processors.

#### Arm Cortex-M4

The target CPU is `cortex-m4`.

- Has DSP extensions - `dsp`
- Optional single precision FPU - `vfp4d16sp` feature flag
  - Vector Floating Point Unit Version 4
  - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)

#### Arm Cortex-M7

The target CPU is `cortex-m7`.

- Has DSP extensions - `dsp`
- Can come with no FPU, a single precision FPU, or a double precision FPU - either `vfp4d16sp` or `vfp4d16` feature flags
  - Vector Floating Point Unit Version 4
  - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
  - The `vfp4d16` feature is like the `vfp4d16sp` but with `+fp64` enabled

Technically the Cortex-M7 has an _FPv5_ FPU, but LLVM doesn't seem to know about it - it is fully compatible with an _VFP4_ FPU though.

#### `thumbv7em-none-eabi` Options

The target CPU options may apply scheduling optimisations and will enable additional features, including the most advanced possible FPU support for that CPU.

| CPU             | Target CPU  | Target Features               |
| --------------- | ----------- | ----------------------------- |
| Cortex-M4       | `cortex-m4` | `+soft-float`                 |
| Cortex-M4F      | `cortex-m4` | None                          |
| Cortex-M7       | `cortex-m7` | `+soft-float`                 |
| Cortex-M7F (SP) | `cortex-m7` | `-fp64` to restrict to SP FPU |
| Cortex-M7F (DP) | `cortex-m7` | None                          |

### `thumbv7em-none-eabihf`

This is suitable for Arm Cortex-M4 and Arm Cortex-M7 processors with FPUs. Floating point function arguments are passed in FPU registers, so the FPU is mandatory.

See [`thumbv7em-none-eabi`](#thumbv7em-none-eabi) for more details.

The target CPU options may apply scheduling optimisations and will enable additional features, including the most advanced possible FPU support for that CPU.

| CPU             | Target CPU  | Target Features               |
| --------------- | ----------- | ----------------------------- |
| Cortex-M4F      | `cortex-m4` | None                          |
| Cortex-M7F (SP) | `cortex-m7` | `-fp64` to restrict to SP FPU |
| Cortex-M7F (DP) | `cortex-m7` | None                          |

### `thumbv8m.base-none-eabi`

This is suitable for Arm Cortex-M23 processor. It supports atomic compare-and-swap.

There is one target CPU option: `cortex-m23`. It may apply scheduling optimisations.

There are no relevant feature flags, and the FPU is not available.

### `thumbv8m.main-none-eabi`

This is suitable for Arm Cortex-M33, Arm Cortex-M55 and Arm Cortex-M85 processors.

#### Arm Cortex-M33

The target CPU is `cortex-m33`.

- Optional DSP extensions - `dsp`
- Optional single precision FP - `fp-armv8d16sp` feature flag
  - Floating Point for ARMv8
  - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)

#### Arm Cortex-M55

The target CPU is `cortex-m55`.

- Has DSP extensions - `dsp`
- Optional double-precision FPU that also supports FP16 - `fp-armv8d16`
  - Floating Point for ARMv8
  - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
- Optional M-Profile Vector Engine that can support either integer-only vectors, or integer/float vectors
  - The appropriate feature for the MVE is either `mve` (integer) or `mve.fp` (float)
  - Also known as _Helium Technology_

#### Arm Cortex-M85

The target CPU is `cortex-m85`.

- Has DSP extensions - `dsp`
- Optional double-precision FPU that also supports FP16 - `fp-armv8d16`
  - Floating Point for ARMv8
  - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
- Optional M-Profile Vector Engine that can support either integer-only vectors, or integer/float vectors
  - The appropriate feature for the MVE is either `mve` (integer) or `mve.fp` (float)
  - Also known as _Helium Technology_

#### `thumbv8m.main-none-eabi` Options

The target CPU options may apply scheduling optimisations and will enable additional features, including the most advanced possible FPU support for that CPU.

| CPU                          | Target CPU   | Target Features            |
| ---------------------------- | ------------ | -------------------------- |
| Cortex-M33                   | `cortex-m33` | `+soft-float,-dsp`         |
| Cortex-M33 with DSP          | `cortex-m33` | `+soft-float`              |
| Cortex-M33F                  | `cortex-m33` | `-dsp`                     |
| Cortex-M33F with DSP         | `cortex-m33` | None                       |
| Cortex-M55                   | `cortex-m55` | `-mve,-mve.fp,+soft-float` |
| Cortex-M55 with Integer MVE  | `cortex-m55` | `-mve.fp,+soft-float`      |
| Cortex-M55F                  | `cortex-m55` | `-mve`                     |
| Cortex-M55F with Integer MVE | `cortex-m55` | `-mve.fp`                  |
| Cortex-M55F with Float MVE   | `cortex-m55` | None                       |
| Cortex-M85                   | `cortex-m85` | `-mve,-mve.fp,+soft-float` |
| Cortex-M85 with Integer MVE  | `cortex-m85` | `-mve.fp,+soft-float`      |
| Cortex-M85F                  | `cortex-m85` | `-mve`                     |
| Cortex-M85F with Integer MVE | `cortex-m85` | `-mve.fp`                  |
| Cortex-M85F with Float MVE   | `cortex-m85` | None                       |

### `thumbv8m.main-none-eabihf`

This is suitable for Arm Cortex-M33, Arm Cortex-M55 and Arm Cortex-M85 processors with FPUs. Floating point function arguments are passed in FPU registers, so the FPU is mandatory.

See [`thumbv8m.main-none-eabi`](#thumbv8mmain-none-eabi) for more details.

The target CPU options may apply scheduling optimisations and will enable additional features, including the most advanced possible FPU support for that CPU.

| CPU                     | Target CPU   | Target Features |
| ----------------------- | ------------ | --------------- |
| Cortex-M33F             | `cortex-m33` | `-dsp`          |
| Cortex-M33F with DSP    | `cortex-m33` | None            |
| Cortex-M55F             | `cortex-m55` | `-mve`          |
| Cortex-M55F Integer MVE | `cortex-m55` | `-mve.fp`       |
| Cortex-M55F Float MVE   | `cortex-m55` | None            |
| Cortex-M85F             | `cortex-m85` | `-mve`          |
| Cortex-M85F Integer MVE | `cortex-m85` | `-mve.fp`       |
| Cortex-M85F Float MVE   | `cortex-m85` | None            |
