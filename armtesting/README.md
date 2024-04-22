# Arm target testing

This repo builds a simple Cortex-M program for a bunch of different targets, target CPUs and target features.

We then diff the assembly to see if the flags made any difference. This tells us which flags are on by default.

## `thumbv6m-none-eabi`

This is suitable for Arm Cortex-M0, Arm Cortex-M0+ and Arm Cortex-M1 processors. It does not support atomic compare-and-swap.

There are no relevant target CPU options.

There are no relevant feature flags, and the FPU is not available.

## `thumbv7m-none-eabi`

This is suitable for Arm Cortex-M3 processor. It supports atomic compare-and-swap.

There are no relevant target CPU options.

There are no relevant feature flags, and the FPU is not available.

## `thumbv7em-none-eabi`

This is suitable for Arm Cortex-M4 and Arm Cortex-M7 processors.

- The Arm Cortex-M4 can come with an optional single precision FPU.
  - The target CPU is `cortex-m4`
  - The appropriate FPU feature is `vfp4d16sp`.
    - Vector Floating Point Unit Version 4
    - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
    - Single Precision (`sp`)
- The Arm Cortex-M7 can come with no FPU, a single precision FPU, or a double precision FPU.
  - The target CPU is `cortex-m7`
  - The appropriate FPU feature is either `vfp4d16sp` or `vfp4d16`.
    - Vector Floating Point Unit Version 4
    - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
    - Double Precision, or Single Precision (`sp`)
  - The `vfp4d16` feature is like the `vfp4d16sp` but with `+fp64` enabled

The target CPU options may apply scheduling optimisations and will enable additional features, including the most advanced possible FPU support for that CPU. Currently it appears to be impossible to disable FPU support when a target CPU option has enabled it, so if you don't have an FPU you cannot use a target CPU setting.

| CPU               | Target CPU  | Target Feature                         |
| ----------------- | ----------- | -------------------------------------- |
| Cortex-M4, no FPU | None        | None                                   |
| Cortex-M4, FPU    | `cortex-m4` | None - FPU is enabled automatically    |
| Cortex-M7, no FPU | None        | None                                   |
| Cortex-M7, SP FPU | `cortex-m7` | `-fp64` to restrict to SP FPU          |
| Cortex-M7, DP FPU | `cortex-m7` | None - DP FPU is enabled automatically |

## `thumbv7em-none-eabihf`

This is suitable for Arm Cortex-M4 and Arm Cortex-M7 processors with FPUs. Floating point function arguments are passed in FPU registers, so the FPU is mandatory.

The target CPU options may apply scheduling optimisations and will enable additional features, including the most advanced possible FPU support for that CPU.

| CPU               | Target CPU  | Target Feature                         |
| ----------------- | ----------- | -------------------------------------- |
| Cortex-M4, FPU    | `cortex-m4` | None - FPU is enabled automatically    |
| Cortex-M7, SP FPU | `cortex-m7` | `-fp64` to restrict to SP FPU          |
| Cortex-M7, DP FPU | `cortex-m7` | None - DP FPU is enabled automatically |

## `thumbv8m.base-none-eabi`

This is suitable for Arm Cortex-M23 processor. It supports atomic compare-and-swap.

There is one target CPU option: `cortex-m23`. It may apply scheduling optimisations.

There are no relevant feature flags, and the FPU is not available.

## `thumbv8m.main-none-eabi`

This is suitable for Arm Cortex-M33, Arm Cortex-M55 and Arm Cortex-M85 processors.

- The Arm Cortex-M33 can come with an optional single precision FPU, and optional DSP extensions.
  - The target CPU is `cortex-m33`
  - The appropriate FPU feature for the Cortex-M33 is `fp-armv8d16sp`.
    - Floating Point for ARMv8
    - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
    - Single Precision (`sp`)
- The Arm Cortex-M55 can come with an optional double-precision FPU that also supports FP16, and with an optional M-Profile Vector Engine that can support either integer-only vectors, or integer/float vectors.
  - The target CPU is `cortex-m55`
  - The appropriate FPU feature for the Cortex-M55 is either `fp-armv8d16`.
    - Floating Point for ARMv8
    - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
  - The appropriate feature for the MVE is either `mve` (integer) or `mve.fp` (float).
- The Arm Cortex-M85 can come with an optional double-precision FPU that also supports FP16, and with an optional M-Profile Vector Engine that can support either integer-only vectors, or integer/float vectors.
  - The target CPU is `cortex-m85`
  - The appropriate FPU feature for the Cortex-M85 is either `fp-armv8d16`.
    - Floating Point for ARMv8
    - Only space for 16 `double`/`f64` values (or 32 `float`/`f32` values)
  - The appropriate feature for the MVE is either `mve` (integer) or `mve.fp` (float).

The target CPU options may apply scheduling optimisations and will enable additional features, including the most advanced possible FPU support for that CPU. Currently it appears to be impossible to disable FPU support when a target CPU option has enabled it, so if you don't have an FPU you cannot use a target CPU setting.

| CPU                                 | Target CPU   | Target Feature                                                                 |
| ----------------------------------- | ------------ | ------------------------------------------------------------------------------ |
| Cortex-M33                          | None         | None                                                                           |
| Cortex-M33 with DSP                 | None         | None                                                                           |
| Cortex-M33 with SP FPU              | `cortex-m33` | None - SP FPU is enabled by default. Unsure if LLVM can emit DSP instructions. |
| Cortex-M33 with DSP and SP FPU      | `cortex-m33` | None - SP FPU is enabled by default. Unsure if LLVM can emit DSP instructions. |
| Cortex-M55                          | None         | None                                                                           |
| Cortex-M55 with Integer MVE         | None         | `+mve`                                                                         |
| Cortex-M55 with FPU                 | `cortex-m55` | `-mve` - disable all MVE                                                       |
| Cortex-M55 with FPU and Integer MVE | `cortex-m55` | `-mve.fp` - disable FP MVE                                                     |
| Cortex-M55 with FPU and Float MVE   | `cortex-m55` | None - DP and FP MVE is enabled by default                                     |
| Cortex-M85                          | None         | None                                                                           |
| Cortex-M85 with Integer MVE         | None         | `+mve`                                                                         |
| Cortex-M85 with FPU                 | `cortex-m85` | `-mve` - disable all MVE                                                       |
| Cortex-M85 with FPU and Integer MVE | `cortex-m85` | `-mve.fp` - disable FP MVE                                                     |
| Cortex-M85 with FPU and Float MVE   | `cortex-m85` | None - DP and FP MVE is enabled by default                                     |

## `thumbv8m.main-none-eabihf`

This is suitable for Arm Cortex-M33, Arm Cortex-M55 and Arm Cortex-M85 processors with FPUs. Floating point function arguments are passed in FPU registers, so the FPU is mandatory.

The target CPU options may apply scheduling optimisations and will enable additional features, including the most advanced possible FPU support for that CPU.

| CPU                                 | Target CPU   | Target Feature                                                                 |
| ----------------------------------- | ------------ | ------------------------------------------------------------------------------ |
| Cortex-M33 with SP FPU              | `cortex-m33` | None - SP FPU is enabled by default. Unsure if LLVM can emit DSP instructions. |
| Cortex-M33 with DSP and SP FPU      | `cortex-m33` | None - SP FPU is enabled by default. Unsure if LLVM can emit DSP instructions. |
| Cortex-M55 with FPU                 | `cortex-m55` | `-mve` - disable all MVE                                                       |
| Cortex-M55 with FPU and Integer MVE | `cortex-m55` | `-mve.fp` - disable FP MVE                                                     |
| Cortex-M55 with FPU and Float MVE   | `cortex-m55` | None - DP and FP MVE is enabled by default                                     |
| Cortex-M85 with FPU                 | `cortex-m85` | `-mve` - disable all MVE                                                       |
| Cortex-M85 with FPU and Integer MVE | `cortex-m85` | `-mve.fp` - disable FP MVE                                                     |
| Cortex-M85 with FPU and Float MVE   | `cortex-m85` | None - DP and FP MVE is enabled by default                                     |
