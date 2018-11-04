#![no_std]

#![feature(abi_x86_interrupt)]
extern crate x86_64;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate vga;
use vga::buffer::BUF_WRITER;
use core::fmt::Write;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    echo!(BUF_WRITER.lock(), "Breakpoint\n{:?}", stack_frame);
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
