// Copyright 2023 Dara Kong
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
