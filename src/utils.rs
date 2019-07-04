use x86_64::instructions;

// wait_for_interrupt permits to halt the
// CPU until a new interrupt has been
// handling
#[allow(dead_code)]
pub fn wait_for_interrupt() -> ! {
    loop {
        instructions::hlt();
    }
}
