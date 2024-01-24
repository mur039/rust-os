#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! { //public, no name mangling, C calling convention
	let vga_buffer = 0xb8000 as *mut u8;
	
	    for (i, &byte) in HELLO.iter().enumerate() {
	        unsafe {
	            *vga_buffer.offset(i as isize * 2) = byte;
	            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; //colour
	        }
	    }
	
	loop{}
}

//this function is called on panic
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! { //non returning function, diverges from here
	loop{}	
}

