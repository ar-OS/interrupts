use interrupts;
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    interrupts::double_fault(stack_frame, error_code);
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    interrupts::breakpoint(stack_frame);
}

pub extern "x86-interrupt" fn timer_handler(_stack_frame: &mut InterruptStackFrame) {
    interrupts::timer();
}

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame) {
    interrupts::keyboard();
}
