// required to unlink std lib so that binary runs on bare metal
#![no_std]

//required to redefine new entry point instead of C runtime
#![no_main]

//required to run tests with custom framework since no_std makes default test framework unusable
#![feature(custom_test_frameworks)]

//create custom test runner
#![test_runner(crate::test_runner)]

//change the name from main to test_main since main is no longer the starting point
#![reexport_test_harness_main = "test_main"]

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

    #[cfg(test)]
    test_main();

    println!("Hello World{}", "! again");
    loop {}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("check");
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}