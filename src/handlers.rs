use core::fmt::Write;
use vga::buffer::BUF_WRITER;
use x86_64::structures::idt::InterruptStackFrame;
use {KEYBOARD_INTERRUPT_ID, PICS, TIMER_INTERRUPT_ID};

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    echo!(
        BUF_WRITER.lock(),
        "Double Fault\nError code: {}\n{:?}",
        error_code,
        stack_frame
    );
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    echo!(BUF_WRITER.lock(), "Breakpoint\n{:?}", stack_frame);
}

pub extern "x86-interrupt" fn timer_handler(_stack_frame: &mut InterruptStackFrame) {
    unsafe { PICS.lock().send_end_interrupt(TIMER_INTERRUPT_ID) };
}

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame) {
    unsafe { PICS.lock().send_end_interrupt(KEYBOARD_INTERRUPT_ID) };
}
