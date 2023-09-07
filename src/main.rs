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

use druid::AppLauncher;
use instant_desktop::{
    app::{self, Delegate, State},
    palette,
    windows::Monitors,
};

fn main() {
    let active_monitors = Monitors::enum_active().list();

    let mut windows: Vec<u32> = active_monitors.iter().map(|mon| mon.id).collect();
    let window = app::window_builder(windows.pop().unwrap(), &active_monitors).unwrap();
    let main_window = window.id;

    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(Delegate::new(main_window, windows))
        .configure_env(|env, _| {
            palette::add_to_env(env);
        })
        .launch(State::new(active_monitors, u32::default()))
        .expect("launch failed");
}
