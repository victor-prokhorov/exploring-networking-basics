/*============================================================================*/
/* no_std: Don't incorporate the standard library as this is a bare-metal     */
/*         based program.                                                     */
/*============================================================================*/
#![no_std]
/*============================================================================*/
/* no_main: Don't use the 'main' function as our entry point.                 */
/*          It is taken care of elsewhere through a modified linker script.   */
/*============================================================================*/
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;
/*============================================================================*/
/* Because there is the small possibility that _start may not run first,      */
/* global assembly is used to ensure that _start is put at the beginning of   */
/* the image.                                                                 */
/*============================================================================*/
mod boot {
    use core::arch::global_asm;
    /*========================================================================*/
    /* glabal_asm macro: says all the code below this line is in the _start   */
    /* section. It can be located in the linker file - linker.ld              */
    /*========================================================================*/
    global_asm!(".section .text._start");
}
/*============================================================================*/
/* no_mangle: Ensures that name _start is manageable, because by default it   */
/* might get 'mangled'. Ensures that in the link environment the symbol name  */
/* is _start.                                                                 */
/*============================================================================*/
#[no_mangle]
/*============================================================================*/
/* Declared as public (extern "C") ensuring the _start symbol is globally     */
/* accessible and the linker can see it at link time, ordered in the right way*/
/* ===========================================================================*/
pub extern "C" fn _start() -> ! {
    /*========================================================================*/
    /* unsafe: The compiler can trust the code and I don't require Rust's     */
    /* memory safety guarantees enforced at compile time for this example.    */
    /*========================================================================*/
    unsafe {
        /*====================================================================*/
        /* Set-up GPIO21 to be an output pin.                                 */
        /*====================================================================*/
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1 << 3);
        /*====================================================================*/
        // Enter an infinite loop that turns an LED on GPIO 21 On and Off.    */
        /*====================================================================*/
        loop {
            /*================================================================*/
            // Turn the LED ON by toggling the output pin HIGH                */
            /*================================================================*/
            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1 << 21);
            /*================================================================*/
            // Wait 50000 ticks, so the LED ON state can be seen.             */
            /*================================================================*/
            for _ in 1..50000 {
                asm!("nop");
            }
            /*================================================================*/
            // Turn the LED OFF by toggling the output pin LOW                */
            /*================================================================*/
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1 << 21);
            /*================================================================*/
            // Wait 50000 ticks, so the LED ON state can be seen.             */
            /*================================================================*/
            for _ in 1..50000 {
                asm!("nop");
            }
        }
    }
}
/*============================================================================*/
/* The panic handler is entered when the OS kernel detects an error and is    */
/* required to ensure error free compilation                                  */
/*============================================================================*/
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
