use core::fmt::Write;

pub use super::riscv64::console::{getchar, putchar};

pub struct DebugConsole;

// Write string through DebugConsole
impl Write for DebugConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.as_bytes().iter().for_each(|&x| putchar(x));
        Ok(())
    }
}

/// Print macro to print polyhal information with newline
#[macro_export]
macro_rules! println {
    () => {
        $crate::arch::console::_print(format_args!("\n"))
    };
    ($fmt: expr $(, $($arg: tt)+)?) => {
        $crate::arch::console::_print(format_args!("{}\n", format_args!($fmt $(, $($arg)+)?)))
    };
}

/// Print macro to print polyhal information with newline
#[macro_export]
macro_rules! print {
    ($fmt: expr $(, $($arg: tt)+)?) => {
        $crate::arch::console::_print(format_args!($fmt $(, $($arg)+)?))
    };
}

/// Print the given arguments
#[inline]
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    DebugConsole.write_fmt(args).expect("can't print arguments");
}

impl log::Log for DebugConsole {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        use log::Level;

        let file = record.module_path();
        let line = record.line();

        let color_code = match record.level() {
            Level::Error => 31u8, // Red
            Level::Warn => 93,    // BrightYellow
            Level::Info => 34,    // Blue
            Level::Debug => 32,   // Green
            Level::Trace => 90,   // BrightBlack
        };
        println!(
            "\u{1B}[{}m\
                    [{}] <{}:{}> {}\
                    \u{1B}[0m",
            color_code,
            record.level(),
            file.unwrap(),
            line.unwrap(),
            record.args()
        );
    }

    fn flush(&self) {}
}

pub(crate) fn log_init() {
    use log::LevelFilter;
    log::set_logger(&DebugConsole).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Debug,
    });
}
