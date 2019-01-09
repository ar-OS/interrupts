use x86_64::structures::idt::ExceptionStackFrame;
use vga::buffer::BUF_WRITER;
use core::fmt::Write;

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    echo!(BUF_WRITER.lock(), "Double Fault\nError code: {}\n{:?}", error_code, stack_frame);
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    echo!(BUF_WRITER.lock(), "Breakpoint\n{:?}", stack_frame);
}




