use x86_64::structures::idt::ExceptionStackFrame;
use vga::buffer::BUF_WRITER;
use core::fmt::Write;
use intel_8259::intel8259::Intel8259;
use spin;

pub const MASTER_OFFSET: u8 = 32;
pub const SLAVE_OFFSET: u8 = MASTER_OFFSET + 8;

lazy_static! {
    pub static ref PICS: spin::Mutex<Intel8259> = spin::Mutex::new(unsafe { Intel8259::new(MASTER_OFFSET, SLAVE_OFFSET) });
}

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    echo!(BUF_WRITER.lock(), "Double Fault\nError code: {}\n{:?}", error_code, stack_frame);
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    echo!(BUF_WRITER.lock(), "Breakpoint\n{:?}", stack_frame);
}




