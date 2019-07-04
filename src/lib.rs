#![no_std]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]
extern crate x86_64;
use x86_64::structures::idt::InterruptDescriptorTable;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate vga;

extern crate spin;

extern crate intel_8259;
use intel_8259::intel8259::Intel8259;

mod handlers;
pub mod utils;

pub const MASTER_OFFSET: u8 = 32;
pub const SLAVE_OFFSET: u8 = MASTER_OFFSET + 8;
pub const TIMER_INTERRUPT_ID: u8 = MASTER_OFFSET;
pub const KEYBOARD_INTERRUPT_ID: u8 = MASTER_OFFSET + 1;

pub static PICS: spin::Mutex<Intel8259> =
    spin::Mutex::new(unsafe { Intel8259::new(MASTER_OFFSET, SLAVE_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // Update the interrupt handlers as the struct is mutable
        idt.breakpoint.set_handler_fn(handlers::breakpoint_handler);
        idt.double_fault.set_handler_fn(handlers::double_fault_handler);
        idt[usize::from(TIMER_INTERRUPT_ID)].set_handler_fn(handlers::timer_handler);
        // Keyboard input related
        idt[usize::from(KEYBOARD_INTERRUPT_ID)].set_handler_fn(handlers::keyboard_handler);
        // Return an immutable version of the struct
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
