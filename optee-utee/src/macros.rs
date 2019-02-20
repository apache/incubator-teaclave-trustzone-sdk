#[macro_export]
macro_rules! trace_print {
    ($($arg:tt)*) => ($crate::Trace::_print(format_args!($($arg)*)));
}

#[macro_export]
#[allow_internal_unstable]
macro_rules! trace_println {
    () => (trace!("\n"));
    ($($arg:tt)*) => ({
        $crate::Trace::_print(format_args_nl!($($arg)*));
    })
}
