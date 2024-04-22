# Arm target testing

This repo builds a simple Cortex-M program for a bunch of different targets, target CPUs and target features.

We then diff the assembly to see if the flags made any difference. This tells us which flags are on by default.

## `thumbv6m-none-eabi`

This is suitable for Arm Cortex-M0, Arm Cortex-M0+ and Arm Cortex-M1 processors. It does not support atomic compare-and-swap.

There are no relevant target CPU options.

There are no relevant feature flags, and the FPU is not available.

## `thumbv7m-none-eabi`

This is suitable for Arm Cortex-M3 processor. It supports atomic compare-and-swap.

### Arm Cortex-M3

The target CPU is `cortex-m3`.

There are no relevant feature flags, and the FPU is not available.

### `thumbv7m-none-eabi` Options

The target CPU options may apply scheduling optimisations but there are no additional features in `cortex-m3` above the architecture baseline.

| CPU       | Target CPU  | Target Features |
| --------- | ----------- | --------------- |
| Cortex-M3 | `cortex-m3` | None            |

## `thumbv7em-none-eabi`

This is suitable for Arm Cortex-M4 and Arm Cortex-M7 processors.

### Arm Cortex-M4

The target CPU is `cortex-m4`.

- Has DSP extensions - `dsp`
- Optional single precision FPU - `vfp4d16sp`
  - Vector Floating Point Unit Version 4
  - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
  - Single Precision (`sp`)

### Arm Cortex-M7

The target CPU is `cortex-m7`.

- Has DSP extensions - `dsp`
- can come with no FPU, a single precision FPU, or a double precision FPU - either `vfp4d16sp` or `vfp4d16`
  - Vector Floating Point Unit Version 4
  - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
  - Double Precision, or Single Precision (`sp`)
  - The `vfp4d16` feature is like the `vfp4d16sp` but with `+fp64` enabled

Technically the Cortex-M7 has an _FPv5_ FPU, but LLVM doesn't seem to know about it - it is fully compatible with an _VFP4_ FPU though.

### `thumbv7em-none-eabi` Options

The target CPU options may apply scheduling optimisations and will enable additional features, including the most advanced possible FPU support for that CPU.

| CPU             | Target CPU  | Target Features               |
| --------------- | ----------- | ----------------------------- |
| Cortex-M4       | `cortex-m4` | `+soft-float`                 |
| Cortex-M4F      | `cortex-m4` | None                          |
| Cortex-M7       | `cortex-m7` | `+soft-float`                 |
| Cortex-M7F (SP) | `cortex-m7` | `-fp64` to restrict to SP FPU |
| Cortex-M7F (DP) | `cortex-m7` | None                          |

## `thumbv7em-none-eabihf`

This is suitable for Arm Cortex-M4 and Arm Cortex-M7 processors with FPUs. Floating point function arguments are passed in FPU registers, so the FPU is mandatory.

See [`thumbv7em-none-eabi`](#thumbv7em-none-eabi) for more details.

The target CPU options may apply scheduling optimisations and will enable additional features, including the most advanced possible FPU support for that CPU.

| CPU             | Target CPU  | Target Features               |
| --------------- | ----------- | ----------------------------- |
| Cortex-M4F      | `cortex-m4` | None                          |
| Cortex-M7F (SP) | `cortex-m7` | `-fp64` to restrict to SP FPU |
| Cortex-M7F (DP) | `cortex-m7` | None                          |

## `thumbv8m.base-none-eabi`

This is suitable for Arm Cortex-M23 processor. It supports atomic compare-and-swap.

There is one target CPU option: `cortex-m23`. It may apply scheduling optimisations.

There are no relevant feature flags, and the FPU is not available.

## `thumbv8m.main-none-eabi`

This is suitable for Arm Cortex-M33, Arm Cortex-M55 and Arm Cortex-M85 processors.

### Arm Cortex-M33

The target CPU is `cortex-m33`.

- Optional DSP extensions - `dsp`
- Optional single precision FP - `fp-armv8d16sp`
  - Floating Point for ARMv8
  - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
  - Single Precision (`sp`)

### Arm Cortex-M55

The target CPU is `cortex-m55`.

- Has DSP extensions - `dsp`
- Optional double-precision FPU that also supports FP16 - `fp-armv8d16`
  - Floating Point for ARMv8
  - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
- Optional M-Profile Vector Engine that can support either integer-only vectors, or integer/float vectors
  - The appropriate feature for the MVE is either `mve` (integer) or `mve.fp` (float)
  - Also known as _Helium Technology_

### Arm Cortex-M85

The target CPU is `cortex-m85`.

- Has DSP extensions - `dsp`
- Optional double-precision FPU that also supports FP16 - `fp-armv8d16`
  - Floating Point for ARMv8
  - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
- Optional M-Profile Vector Engine that can support either integer-only vectors, or integer/float vectors
  - The appropriate feature for the MVE is either `mve` (integer) or `mve.fp` (float)
  - Also known as _Helium Technology_

### `thumbv8m.main-none-eabi` Options

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

## `thumbv8m.main-none-eabihf`

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
