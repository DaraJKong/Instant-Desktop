#![windows_subsystem = "windows"]

use widestring::U16CString;
use windows::{
    core::PCWSTR,
    w,
    Win32::{
        Foundation::{HINSTANCE, HWND},
        UI::{Shell, WindowsAndMessaging::MB_ICONASTERISK, HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE}},
    },
};

use instant_desktop::Monitors;

fn main() {
    unsafe {
        SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE).expect("SetProcessDpiAwareness should succeed");
    }

    let active_monitors_list = Monitors::enum_active();

    let wide_text = U16CString::from_str(
        active_monitors_list
            .list()
            .iter()
            .map(|mon| {
                format!(
                    "{}: {} x {}; ({}, {}, {}, {})",
                    mon.id(),
                    mon.width(),
                    mon.height(),
                    mon.left(),
                    mon.top(),
                    mon.right() - 1,
                    mon.bottom() - 1
                )
            })
            .collect::<Vec<String>>()
            .join("\n"),
    )
    .expect("conversion from str to U16CString should work");

    let text_ptr = PCWSTR::from_raw(wide_text.as_ptr());

    unsafe {
        let _: i32 = Shell::ShellMessageBoxW(
            HINSTANCE::default(),
            HWND::default(),
            text_ptr,
            w!("Instant Desktop"),
            MB_ICONASTERISK,
        );
    }
}
