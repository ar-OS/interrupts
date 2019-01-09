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
use handlers::{breakpoint_handler, double_fault_handler};

pub const MASTER_OFFSET: u8 = 32;
pub const SLAVE_OFFSET: u8 = MASTER_OFFSET + 8;

lazy_static! {
    pub static ref PICS: spin::Mutex<Intel8259> = spin::Mutex::new(unsafe { Intel8259::new(MASTER_OFFSET, SLAVE_OFFSET) });
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
