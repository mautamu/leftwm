//! Utilities for sending notifications and log messages to end user.

/// This macro is exported to the root of the crate and may be called with `leftwm_core::log!();`.
/// It follows the log convention, with levels `trace`, `debug`, `info`, `warn`, and `error`. It also has a
/// notification-only `notify` level. It will send a notification for errors and warnings only.
///
/// # Errors
///
/// Most errors should occur at compile time if they exist. Otherwise they're sent to `log::debug` in
/// the `notify!()` macro.
#[macro_export]
macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => ({
        match $lvl{
            "info" => {$crate::log_log!($crate::Level::Info, $($arg)+);},
            "warn" => {$crate::log_log!($crate::Level::Warn, $($arg)+); $crate::notify!($($arg)+);},
            "error" => {$crate::log_log!($crate::Level::Error, $($arg)+); $crate::notify!($($arg)+);},
            "trace" => {$crate::log_log!($crate::Level::Trace, $($arg)+);},
            "debug" => {$crate::log_log!($crate::Level::Debug, $($arg)+);},
            "notify" => {$crate::notify!($($arg)+);},
            _ => $crate::log_log!($crate::Level::Info,$($arg)+),
        };
    });
}

/// This macro sends a user-facing notification using the `notify-rs` crate which calls the
/// org.freedesktop.Notifications library.
///
/// # Errors
///
/// If notification fails, an error will be sent to the `Debug` level for `log::log!();`
#[macro_export]
macro_rules! notify {
    ($($arg:tt)+) => ({
        let o_string:String = $crate::__format!($($arg)+).as_str().unwrap_or("Unknown LeftWM Error").to_string();
        match $crate::Notification::new()
            .summary("LeftWM Notice")
            .body(&o_string)
            .icon("leftwm")
            .show(){
                Ok(_) => {
                    // Nothing to do
                },
                Err(_) => {
                    // We should probably do something about this
                    $crate::log_log!($crate::Level::Debug, "Notification failure");
                }
            }
    });
}

/// Format macro invoker
/// Formats arguments to pass as strings later on.
#[macro_export]
macro_rules! __format {
  ($($args:tt)*) => {
    format_args!($($args)*)
   };
}
