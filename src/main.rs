#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

// function selector's registry addresses
const GPIO_FSEL0: i32 = 0x03f20_0000;
const GPIO_FSEL1: i32 = 0x03f20_0004;
const GPIO_FSEL2: i32 = 0x03f20_0008;
const GPIO_FSEL3: i32 = 0x03f20_000c;
const GPIO_FSEL4: i32 = 0x03f20_0010;
const GPIO_FSEL5: i32 = 0x03f20_0010;

// GPIO pin output registers
const GPIO_SET0: i32 = 0x03f20_001c;
const GPIO_CLR0: i32 = 0x03f20_0028;
const GPIO_SET1: i32 = 0x03f20_0020;
const GPIO_CLR1: i32 = 0x03f20_002c;

// alias type to abstract HIGH and LOW values
type GPIOValue = u32;

const HIGH: GPIOValue = 1;
const LOW: GPIOValue = 0;


mod boot {
    use core::arch::global_asm;
    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let pin: u32 = 21;

    GPIO::set_output(pin);
    let cycles: u32 = 100_000;

    loop {
        GPIO::set(pin, HIGH);
        delay(cycles * 5);

        GPIO::set(pin, LOW);
        delay(cycles);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


struct GPIO;

impl GPIO {
    
    pub fn set_output(pin: u32) {
        if pin > 53 {
            panic!("unsupported gpio pin")
        }
        let reg = pin / 10;
        let register = match reg {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            3 => GPIO_FSEL3,
            4 => GPIO_FSEL4,
            5 => GPIO_FSEL5,
            _ => panic!("fatal error while selecting register"),
        };

        let mut val: u32 = 0;

        unsafe {
            val = core::ptr::read_volatile(register as *mut u32);
        }

        let mut mask: u32 = 0b111;
        let pinnum = pin % 10;
        mask = mask << pinnum * 3;

        val = val & !(mask);

        val |= 1 << pinnum * 3;

        unsafe {
            core::ptr::write_volatile(register as *mut u32, val);
        }
    }

    pub fn set(pin: u32, state: GPIOValue) {
        if pin > 53 {
            panic!("unsupported gpio pin")
        }

        let mem_address: i32;

        if pin <= 31 {
            if state == LOW {
                mem_address = GPIO_CLR0;
            } else {
                mem_address = GPIO_SET0;
            }
        } else {
            if state == LOW {
                mem_address = GPIO_CLR1;
            } else {
                mem_address = GPIO_SET1;
            }
        }

        let mut bitposition: u32 = pin;
        if pin > 31 {
            bitposition = pin - 32;
        }

        let mut val: u32 = 0;

        unsafe {
            val = core::ptr::read_volatile(mem_address as *mut u32);
        }

        val |= 1 << bitposition;

        unsafe {
            core::ptr::write_volatile(mem_address as *mut u32, val);
        }
    }
}

fn delay(cycles: u32) {
    for _ in 1..cycles {
        unsafe {
            asm!("nop");
        }
    }
}
