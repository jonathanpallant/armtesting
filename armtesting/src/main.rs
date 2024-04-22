#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;

#[inline(never)]
fn hidenext() -> i32 {
    static ATOMIC_NUMBER: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(0);
    ATOMIC_NUMBER.fetch_add(1, core::sync::atomic::Ordering::Relaxed)
}

#[inline(never)]
fn next() -> i32 {
    core::hint::black_box(hidenext())
}

#[entry]
fn main() -> ! {
    let mut data1 = [
        next() as u8,
        next() as u8,
        next() as u8,
        next() as u8,
        next() as u8,
        next() as u8,
        next() as u8,
        next() as u8,
    ];
    let data2 = [
        next() as u8,
        next() as u8,
        next() as u8,
        next() as u8,
        next() as u8,
        next() as u8,
        next() as u8,
        next() as u8,
    ];
    int_maths(&mut data1, &data2);
    let sum: u8 = data1.iter().sum();

    let mut fdata1 = [
        next() as f32,
        next() as f32,
        next() as f32,
        next() as f32,
        next() as f32,
        next() as f32,
        next() as f32,
        next() as f32,
    ];
    let fdata2 = [
        next() as f32,
        next() as f32,
        next() as f32,
        next() as f32,
        next() as f32,
        next() as f32,
        next() as f32,
        next() as f32,
    ];
    f32_maths(&mut fdata1, &fdata2);
    let fsum: f32 = fdata1.iter().sum();

    let mut f64data1 = [
        next() as f64,
        next() as f64,
        next() as f64,
        next() as f64,
        next() as f64,
        next() as f64,
        next() as f64,
        next() as f64,
    ];
    let f64data2 = [
        next() as f64,
        next() as f64,
        next() as f64,
        next() as f64,
        next() as f64,
        next() as f64,
        next() as f64,
        next() as f64,
    ];
    f64_maths(&mut f64data1, &f64data2);
    let f64sum: f64 = f64data1.iter().sum();

    let total = sum as f64 + fsum as f64 + f64sum;
    if total > 0.0 {
        panic!("a");
    } else {
        panic!("b");
    }
}

#[inline(never)]
pub fn int_maths(data1: &mut [u8; 8], data2: &[u8; 8]) {
    for i in 0..8 {
        data1[i] += data2[i];
    }
}

#[inline(never)]
pub fn f32_maths(data1: &mut [f32; 8], data2: &[f32; 8]) {
    for i in 0..8 {
        data1[i] += data2[i];
    }
}

#[inline(never)]
pub fn f64_maths(data1: &mut [f64; 8], data2: &[f64; 8]) {
    for i in 0..8 {
        data1[i] += data2[i];
    }
}
