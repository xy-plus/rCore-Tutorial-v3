#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::ptr;
use user_lib::{mmap, munmap};

#[no_mangle]
fn main() -> i32 {
    let start: usize = 0x10000000;
    let len: usize = 5000;
    let prot: usize = 1;
    let map_len = mmap(start, len, prot);
    assert_eq!(len, map_len as usize);
    let mut addr: *mut u8 = start as *mut u8;
    for i in start..(start + len) {
        let mut addr: *mut u8 = i as *mut u8;
        unsafe {
            *addr = i as u8;
        }
    }
    for i in start..(start + len) {
        let mut addr: *mut u8 = i as *mut u8;
        unsafe {
            assert_eq!(*addr, i as u8);
        }
    }
    println!("Test mmap OK!");
    let unmap_len = munmap(start, len);
    assert_eq!(len, unmap_len as usize);
    unsafe {
        let mut addr: *mut u8 = start as *mut u8;
        *addr = 1;
    }
    // if core dumped here, unmap succeed
    0
}
