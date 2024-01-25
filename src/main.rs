#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
mod vga_buffer;
mod serial;
use core::panic::PanicInfo;
use bootloader::BootInfo;

use crate::serial::_read_byte;

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! { //public, no name mangling, C calling convention

	//const move_cursor : &[u8] = b"ESC[{line};{column}H" ;

	for (i, region)  in _boot_info.memory_map.iter().enumerate()  {
		serial_println!("{:2}: {1:#x}...{2:#x}->{3:?}, size : {4:#x}", 	
		i, region.range.start_addr(), region.range.end_addr(), region.region_type, (region.range.end_addr() - region.range.start_addr())
		);
	}
	serial_print!("\x1b[999;999H"); //moving cursor to 999
	serial_print!("\x1b[6n"); //request cursor position (reports as ESC[ #;#R) line;col

	{	//:( bu nedir yahu
		let mut index : usize = 0;
		let mut ch = _read_byte() as char;
		let mut report_buffer : [char; 10] = ['\0'; 10];

		while ch != 'R' { //reading from serial port until character 'R' is received
			report_buffer[index] = ch as char;
			index += 1;
			ch = _read_byte() as char;
		}

		report_buffer.iter().for_each(|x| {
			println!("{} : {}", *x as u8, *x as char);
		});

		let mut _col = 0;
		let mut _line = 0;
		
		let mut _iter1 :usize = 0;
		let mut _iter2 :usize = 0;
		
		let mut state : bool = false;
		report_buffer.iter_mut().enumerate().for_each( |(i ,x)| {
			
			match *x {
				'0'..'9' =>  //entry to number
					{
						match state {
							false => _iter1 = i,
							true => _iter2 =  1 + i
						}
						state = true;
					},
				_ => state = false // not in a number
			}

		}
		); // ?

		println!("{:?}", report_buffer[_iter1.._iter2].as_mut());

		
		
	}

	loop{
		//serial_println!("{}\r", _read_byte() as char); //simple echo?
	}
}

//this function is called on panic
#[panic_handler]
fn panic(info : &PanicInfo) -> ! { //non returning function, diverges from here
	println!("{}", info);
	loop{}	
}

