// required to unlink std lib so that binary runs on bare metal
#![no_std]

//required to redefine new entry point instead of C runtime
#![no_main]

// required to add panic handler removed by no_std
use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}
