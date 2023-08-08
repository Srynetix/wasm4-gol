/// Print a line.
#[macro_export]
macro_rules! println_w4 {
    ($($args:tt)*) => {
        let mut writer = $crate::arrayvec::ArrayString::<128>::new();
        ::core::fmt::write(&mut writer, format_args!($($args)*)).unwrap();
        $crate::wasm4::trace(writer)
    };
}
