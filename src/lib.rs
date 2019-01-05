#![no_std]

#![feature(abi_x86_interrupt)]
extern crate x86_64;
use x86_64::structures::idt::InterruptDescriptorTable;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate vga;

mod handlers;
use handlers::{breakpoint_handler, double_fault_handler};

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
