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

use druid::{
    widget::{Align, Button, Label, SizedBox},
    AppDelegate, AppLauncher, Application, Data, DelegateCtx, Env, Event, KbKey, Point, Size,
    Widget, WindowDesc, WindowId, WindowState,
};

use instant_desktop::windows::Monitors;

#[derive(Clone, Data)]
struct State {
    monitors: Monitors,
}

fn main() {
    let active_monitors = Monitors::enum_active();

    let mut windows: Vec<u32> = active_monitors.list().iter().map(|mon| mon.id()).collect();
    let window = window_builder(windows.pop().unwrap(), &active_monitors).unwrap();
    let main_window = window.id;

    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(Delegate {
            main_window,
            windows,
        })
        .launch(State {
            monitors: active_monitors,
        })
        .expect("launch failed");
}

fn window_builder(id: u32, monitors: &Monitors) -> Option<WindowDesc<State>> {
    let list = monitors.list();
    let monitor = list.iter().find(|&mon| mon.id() == id);

    if let Some(monitor) = monitor {
        Some(
            WindowDesc::new(ui_builder(id))
                .title("Instant Desktop")
                .window_size(Size::new(
                    monitor.work_width().into(),
                    monitor.work_height().into(),
                ))
                .set_position(Point::new(
                    monitor.work_left().into(),
                    monitor.work_top().into(),
                ))
                .show_titlebar(false)
                .resizable(false)
                .set_window_state(WindowState::Maximized),
        )
    } else {
        None
    }
}

fn ui_builder(id: u32) -> impl Widget<State> {
    let id_label = Label::dynamic(move |data: &State, _| {
        if let Some(mon) = data.monitors.list().iter().find(|&mon| mon.id() == id) {
            mon.id().to_string()
        } else {
            String::from("error: monitor not found")
        }
    })
    .with_text_size(150.0);

    Align::centered(
        SizedBox::new(Button::from_label(id_label))
            .width(200.0)
            .height(200.0),
    )
}

struct Delegate {
    main_window: WindowId,
    windows: Vec<u32>,
}

impl AppDelegate<State> for Delegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx,
        window_id: WindowId,
        event: Event,
        data: &mut State,
        env: &Env,
    ) -> Option<Event> {
        match &event {
            Event::WindowConnected => {
                if window_id == self.main_window {
                    for id in &self.windows {
                        let window = window_builder(*id, &data.monitors).unwrap();
                        ctx.new_window(window);
                    }
                }
            }
            Event::KeyDown(event) => match event.key {
                KbKey::Escape => Application::global().quit(),
                _ => (),
            },
            _ => (),
        }

        Some(event)
    }
}
