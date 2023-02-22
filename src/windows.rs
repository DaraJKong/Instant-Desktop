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

use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{BOOL, LPARAM, RECT},
        Graphics::Gdi::{
            self, DISPLAY_DEVICEW, DISPLAY_DEVICE_ACTIVE, HDC, HMONITOR, MONITORINFOEXW,
        },
        // UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE},
    },
};

use druid::{im::Vector, Data};
use widestring::U16CString;
use windows::{
    w,
    Win32::{
        Foundation::{HINSTANCE, HWND},
        UI::{Shell, WindowsAndMessaging::MB_ICONASTERISK},
    },
};

#[derive(Clone, Data)]
pub struct Monitor {
    id: u32,
    #[data(ignore)]
    display_monitor_handle: HMONITOR,
    #[data(ignore)]
    monitor_info: MONITORINFOEXW,
}

impl Monitor {
    pub fn new(id: u32) -> Self {
        Monitor {
            id,
            display_monitor_handle: HMONITOR::default(),
            monitor_info: MONITORINFOEXW::default(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn left(&self) -> i32 {
        self.monitor_info.monitorInfo.rcMonitor.left
    }

    pub fn top(&self) -> i32 {
        self.monitor_info.monitorInfo.rcMonitor.top
    }

    pub fn right(&self) -> i32 {
        self.monitor_info.monitorInfo.rcMonitor.right
    }

    pub fn bottom(&self) -> i32 {
        self.monitor_info.monitorInfo.rcMonitor.bottom
    }

    pub fn width(&self) -> i32 {
        self.right() - self.left()
    }

    pub fn height(&self) -> i32 {
        self.bottom() - self.top()
    }

    pub fn work_left(&self) -> i32 {
        self.monitor_info.monitorInfo.rcWork.left
    }

    pub fn work_top(&self) -> i32 {
        self.monitor_info.monitorInfo.rcWork.top
    }

    pub fn work_right(&self) -> i32 {
        self.monitor_info.monitorInfo.rcWork.right
    }

    pub fn work_bottom(&self) -> i32 {
        self.monitor_info.monitorInfo.rcWork.bottom
    }

    pub fn work_width(&self) -> i32 {
        self.work_right() - self.work_left()
    }

    pub fn work_height(&self) -> i32 {
        self.work_bottom() - self.work_top()
    }

    pub fn info_str(&self) -> String {
        format!(
            "{}: {} x {}; ({}, {}, {}, {})",
            self.id(),
            self.width(),
            self.height(),
            self.left(),
            self.top(),
            self.right() - 1,
            self.bottom() - 1
        )
    }
}

#[derive(Clone, Data)]
pub struct Monitors {
    list: Vector<Monitor>,
}

impl Monitors {
    pub fn new() -> Self {
        Self {
            list: Vector::new(),
        }
    }

    pub fn enum_active() -> Self {
        /*unsafe {
            SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE)
                .expect("SetProcessDpiAwareness should succeed");
        }*/

        let mut active_monitors_list = Monitors::new();

        let mut display_device = DISPLAY_DEVICEW::default();
        let device_name_ptr = PCWSTR::from_raw(&mut display_device.DeviceName as *mut _);

        let mut i = 0;

        loop {
            display_device.cb = 840;

            let result =
                unsafe { Gdi::EnumDisplayDevicesW(PCWSTR::null(), i, &mut display_device, 0) };

            if !result.as_bool() {
                break;
            }

            let handle =
                unsafe { Gdi::CreateDCW(device_name_ptr, device_name_ptr, PCWSTR::null(), None) };

            if !handle.is_invalid() && display_device.StateFlags & DISPLAY_DEVICE_ACTIVE == 1 {
                let mut monitor = Monitor::new(i);

                monitor.monitor_info.szDevice = display_device.DeviceName;

                let monitor_ptr = LPARAM(&mut monitor as *mut _ as isize);

                unsafe {
                    Gdi::EnumDisplayMonitors(
                        HDC::default(),
                        None,
                        Some(monitor_enum_proc),
                        monitor_ptr,
                    );
                }

                if monitor.monitor_info.monitorInfo.cbSize != 0 {
                    active_monitors_list.add(monitor);
                }
            }

            i += 1;
        }

        active_monitors_list
    }

    pub fn add(&mut self, monitor: Monitor) {
        self.list.push_back(monitor);
    }

    pub fn list(&self) -> Vector<Monitor> {
        self.list.clone()
    }

    pub fn display_list(&self) {
        let wide_text = U16CString::from_str(
            self.list()
                .iter()
                .map(|mon| mon.info_str())
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
}

unsafe extern "system" fn monitor_enum_proc(
    display_monitor_handle: HMONITOR,
    _: HDC,
    _: *mut RECT,
    monitor_ptr: LPARAM,
) -> BOOL {
    let mut monitor_info = MONITORINFOEXW::default();
    monitor_info.monitorInfo.cbSize = 104;

    let result = Gdi::GetMonitorInfoW(
        display_monitor_handle,
        &mut monitor_info.monitorInfo as *mut _,
    );

    let mut continue_enum = BOOL::from(true);

    if result.as_bool() {
        let monitor = (monitor_ptr.0 as *mut Monitor).as_mut();

        if let Some(monitor) = monitor {
            if monitor_info.szDevice == monitor.monitor_info.szDevice {
                monitor.display_monitor_handle = display_monitor_handle;
                monitor.monitor_info = monitor_info;

                continue_enum = BOOL::from(false);
            }
        }
    }

    continue_enum
}
