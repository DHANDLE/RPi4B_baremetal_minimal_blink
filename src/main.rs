#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

// 0x7enn_nnnn is mapped to 0xFEnn_nnnn
const GPIO_BASE_ADDR: u32 = 0xFE20_0000;
const GPFSEL2: u32 = GPIO_BASE_ADDR + 0x08;
const GPSET0: u32 = GPIO_BASE_ADDR + 0x1C;
const GPCLR0: u32 = GPIO_BASE_ADDR + 0x28;

#[link_section = ".text.boot"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Write to GPFSEL2 to turn PIN21 into an output.
        core::ptr::write_volatile(GPFSEL2 as *mut u32, 1 << 3);

        loop {
            // Write to GPSET0 to set PIN21 to 1.
            core::ptr::write_volatile(GPSET0 as *mut u32, 1 << 21);

            sleep(500000);

            // Write to GPCLR0 to set PIN21 to 0.
            core::ptr::write_volatile(GPCLR0 as *mut u32, 1 << 21);

            sleep(500000);
        }
    }
}

unsafe fn sleep(cycles: u32) {
    for _ in 1..cycles {
        asm!("nop");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
