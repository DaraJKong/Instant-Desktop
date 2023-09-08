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

use druid::{
    im::Vector,
    widget::{Controller, EnvScope, Label, SizedBox},
    AppDelegate, Application, Color, Data, DelegateCtx, Env, Event, EventCtx, KbKey, Point, Size,
    Widget, WidgetExt, WindowDesc, WindowId,
};

use crate::windows::Monitor;
use crate::{config::Config, connection, palette};

#[derive(Clone, Data)]
pub struct State {
    config: Config,
    monitors: Vector<Monitor>,
    hovered_id: u32,
}

impl State {
    pub fn new(config: Config, monitors: Vector<Monitor>, hovered_id: u32) -> Self {
        Self {
            config,
            monitors,
            hovered_id,
        }
    }

    pub fn get_selected(&self) -> Vec<u32> {
        self.monitors
            .iter()
            .filter_map(|monitor| {
                if monitor.selected {
                    Some(monitor.id)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn window_builder(
    config: &Config,
    id: u32,
    monitors: &Vector<Monitor>,
) -> Option<WindowDesc<State>> {
    let list = monitors;
    let monitor = list.iter().find(|&mon| mon.id == id);

    if let Some(monitor) = monitor {
        let (width, height, left, top) = if config.fullscreen {
            (
                monitor.width(),
                monitor.height(),
                monitor.left(),
                monitor.top(),
            )
        } else {
            (
                monitor.work_width(),
                monitor.work_height(),
                monitor.work_left(),
                monitor.work_top(),
            )
        };

        Some(
            WindowDesc::new(ui_builder(id))
                .title("Instant Desktop")
                .window_size(Size::new(width.into(), height.into()))
                .set_position(Point::new(left.into(), top.into()))
                .show_titlebar(false)
                .resizable(false)
                .set_always_on_top(true),
        )
    } else {
        None
    }
}

fn ui_builder(id: u32) -> impl Widget<State> {
    let id_label = Label::dynamic(move |data: &State, _| {
        if let Some(mon) = data.monitors.iter().find(|&mon| mon.id == id) {
            mon.id().to_string()
        } else {
            String::from("error: monitor not found")
        }
    })
    .with_text_size(200.0)
    .with_text_color(palette::ID_TEXT_COLOR)
    .center()
    .background(palette::ID_BACKGROUND_COLOR);

    let id_box = SizedBox::new(id_label).width(300.0).height(300.0).center();

    EnvScope::new(
        move |env, data| {
            if let Some(monitor) = data.monitors.iter().find(|monitor| monitor.id == id) {
                if monitor.selected {
                    if monitor.id == data.hovered_id {
                        env.set(palette::ID_BACKGROUND_COLOR, Color::WHITE);
                        env.set(palette::ID_TEXT_COLOR, env.get(palette::DARK));
                    } else {
                        env.set(palette::ID_BACKGROUND_COLOR, env.get(palette::DARK));
                        env.set(palette::ID_TEXT_COLOR, Color::WHITE);
                    }

                    env.set(palette::MONITOR_BACKGROUND_COLOR, env.get(palette::PRIMARY));
                } else {
                    if monitor.id == data.hovered_id {
                        env.set(palette::MONITOR_BACKGROUND_COLOR, env.get(palette::DARK));
                        env.set(
                            palette::ID_BACKGROUND_COLOR,
                            env.get(palette::LIGHT_HOVERED),
                        );
                    } else {
                        env.set(palette::MONITOR_BACKGROUND_COLOR, Color::BLACK);
                        env.set(palette::ID_BACKGROUND_COLOR, env.get(palette::LIGHT));
                    }

                    env.set(palette::ID_TEXT_COLOR, Color::WHITE);
                }
            }
        },
        SizedBox::new(id_box)
            .expand()
            .background(palette::MONITOR_BACKGROUND_COLOR)
            .controller(Hoverable(id))
            .on_click(move |_, data, _| {
                for i in 0..data.monitors.len() {
                    if data.monitors[i].id == id {
                        data.monitors[i].selected = !data.monitors[i].selected;
                    }
                }
            }),
    )
}

struct Hoverable(u32);

impl<W: Widget<State>> Controller<State, W> for Hoverable {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut State,
        env: &Env,
    ) {
        match event {
            Event::MouseMove(_) if ctx.is_hot() => data.hovered_id = self.0,
            _ => (),
        }

        child.event(ctx, event, data, env);
    }
}

pub struct Delegate {
    main_window: WindowId,
    windows: Vec<u32>,
}

impl Delegate {
    pub fn new(main_window: WindowId, windows: Vec<u32>) -> Self {
        Self {
            main_window,
            windows,
        }
    }
}

impl AppDelegate<State> for Delegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx,
        window_id: WindowId,
        event: Event,
        data: &mut State,
        _env: &Env,
    ) -> Option<Event> {
        match &event {
            Event::WindowConnected => {
                if window_id == self.main_window {
                    for id in &self.windows {
                        let window = window_builder(&data.config, *id, &data.monitors).unwrap();
                        ctx.new_window(window);
                    }
                }
            }
            Event::KeyDown(event) => match event.key {
                KbKey::Escape | KbKey::Backspace | KbKey::Delete => Application::global().quit(),
                KbKey::Enter => {
                    connection::start_rdc_session(&data.config, data.get_selected());
                    Application::global().quit();
                }
                _ => (),
            },
            _ => (),
        }

        Some(event)
    }
}
