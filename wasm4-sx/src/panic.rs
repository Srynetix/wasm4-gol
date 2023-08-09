/// Setup panic handler for WASM-4.
#[macro_export]
macro_rules! setup_panic_handler_w4 {
    () => {
        #[cfg(not(test))]
        #[panic_handler]
        fn panic(info: &::core::panic::PanicInfo) -> ! {
            $crate::println_w4!("{}", info);
            loop {}
        }
    };
}
