#![allow(unused, non_snake_case)]

#[no_mangle]
pub extern "C" fn blit(sprite: *const u8, x: i32, y: i32, width: u32, height: u32, flags: u32) {
    // Stub
}

#[no_mangle]
pub extern "C" fn blitSub(
    sprite: *const u8,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    src_x: u32,
    src_y: u32,
    stride: u32,
    flags: u32,
) {
    // Stub
}

#[no_mangle]
pub extern "C" fn line(x1: i32, y1: i32, x2: i32, y2: i32) {
    // Stub
}

#[no_mangle]
pub extern "C" fn oval(x: i32, y: i32, width: u32, height: u32) {
    // Stub
}

#[no_mangle]
pub extern "C" fn rect(x: i32, y: i32, width: u32, height: u32) {
    // Stub
}

#[no_mangle]
pub extern "C" fn textUtf8(text: *const u8, length: usize, x: i32, y: i32) {
    // Stub
}

#[no_mangle]
pub extern "C" fn vline(x: i32, y: i32, len: u32) {
    // Stub
}

#[no_mangle]
pub extern "C" fn hline(x: i32, y: i32, len: u32) {
    // Stub
}

#[no_mangle]
pub extern "C" fn tone(frequency: u32, duration: u32, volume: u32, flags: u32) {
    // Stub
}

#[no_mangle]
pub extern "C" fn diskr(dest: *mut u8, size: u32) -> u32 {
    // Stub
    0
}

#[no_mangle]
pub extern "C" fn diskw(src: *const u8, size: u32) -> u32 {
    // Stub
    0
}

#[no_mangle]
pub extern "C" fn traceUtf8(trace: *const u8, length: usize) {
    // Stub
}
