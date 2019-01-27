use x86_64::structures::idt::ExceptionStackFrame;
use vga::buffer::BUF_WRITER;
use core::fmt::Write;
use {PICS, TIMER_INTERRUPT_ID};

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    echo!(BUF_WRITER.lock(), "Double Fault\nError code: {}\n{:?}", error_code, stack_frame);
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    echo!(BUF_WRITER.lock(), "Breakpoint\n{:?}", stack_frame);
}

pub extern "x86-interrupt" fn timer_handler(stack_frame: &mut ExceptionStackFrame) {
    echo!(BUF_WRITER.lock(), ".");
    unsafe { PICS.lock().send_end_interrupt(TIMER_INTERRUPT_ID) };
}

