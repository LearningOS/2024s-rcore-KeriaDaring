#![no_std]
#![no_main]
use sbi::shutdown;

mod console;
mod lang_items;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() {
    // loop {

    // };
    clear_bss();
    println!("你成功啦孩子");
    shutdown();
}
