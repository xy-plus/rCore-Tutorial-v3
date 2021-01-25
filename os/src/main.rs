#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(const_in_array_repeat_expressions)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod console;
mod cpu;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod config;
mod task;
mod timer;
mod mm;
mod fs;
mod drivers;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

const BOOT_HART_ID: usize = 0;

#[no_mangle]
pub fn rust_main(hartid: usize, _device_tree_paddr: usize) -> ! {
    unsafe {
        cpu::set_cpu_id(hartid);
    }
    if hartid == BOOT_HART_ID {
        clear_bss();
        info!("Hello, world!");
        mm::init();
        mm::remap_test();
        task::add_initproc();
        info!("after initproc!");
        trap::init();
        trap::enable_timer_interrupt();
        timer::set_next_trigger();
        fs::list_apps();
        cpu::broadcast_ipi(); // wake other core
        task::run_tasks();
        panic!("Unreachable in rust_main!");
    } else {
        others_main(hartid);
    }
}

fn others_main(hartid: usize) -> ! {
    info!("Hello RISCV! in hart {}", hartid);
    trap::init();
    crate::mm::KERNEL_SPACE.clone().lock().activate();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_tasks();
    panic!("Unreachable in other_main!");
}
