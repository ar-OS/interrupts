use core::fmt::Write;
use keyboard::{set::Set1, Keyboard};
use vga::buffer::BUF_WRITER;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;
use {KEYBOARD_INTERRUPT_ID, PICS, TIMER_INTERRUPT_ID};

const KEYBOARD_CONTROLLER_PORT: u16 = 0x60;

pub fn double_fault(stack_frame: &mut InterruptStackFrame, error_code: u64) {
    echo!(
        "Double Fault\nError code: {}\n{:?}",
        error_code,
        stack_frame
    );
}

pub fn breakpoint(stack_frame: &mut InterruptStackFrame) {
    echo!("Breakpoint\n{:?}", stack_frame);
}

pub fn timer() {
    echo!("{}", ".");
    unsafe { PICS.lock().send_end_interrupt(TIMER_INTERRUPT_ID) };
}

pub fn keyboard() {
    let mut keyboard_port: Port<u8> = Port::new(KEYBOARD_CONTROLLER_PORT);
    let mut keyboard: Keyboard<Set1> = Keyboard::new(Set1 {});
    let scancode: u8 = unsafe { keyboard_port.read() };
    let key = keyboard.push(scancode);
    if key.is_some() {
        let keycode = key.unwrap().keycode();
        if keycode.is_some() {
            echo!("{}", keycode.unwrap());
        }
    }
    unsafe { PICS.lock().send_end_interrupt(KEYBOARD_INTERRUPT_ID) };
}
