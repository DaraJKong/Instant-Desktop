use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{BOOL, LPARAM, RECT},
        Graphics::Gdi::{
            self, DISPLAY_DEVICEW, DISPLAY_DEVICE_ACTIVE, HDC, HMONITOR, MONITORINFOEXW,
        },
    },
};

#[derive(Clone, Debug)]
pub struct Monitor {
    id: u32,
    display_monitor_handle: HMONITOR,
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
}

pub struct Monitors {
    list: Vec<Monitor>,
}

impl Monitors {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn enum_active() -> Self {
        let mut active_monitors_list = Monitors::new();

        // TODO: move safe stuff out of the unsafe{} block

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
        self.list.push(monitor);
    }

    pub fn list(&self) -> Vec<Monitor> {
        self.list.clone()
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
