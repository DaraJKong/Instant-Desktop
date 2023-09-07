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

use druid::{Color, Env, Key};

pub const PRIMARY: Key<Color> = Key::new("instant_desktop.palette.primary");
pub const LIGHT: Key<Color> = Key::new("instant_desktop.palette.light");
pub const LIGHT_HOVERED: Key<Color> = Key::new("instant_desktop.palette.light_hovered");
pub const DARK: Key<Color> = Key::new("instant_desktop.palette.dark");

pub const MONITOR_BACKGROUND_COLOR: Key<Color> =
    Key::new("instant_desktop.palette.monitor_background_color");
pub const ID_BACKGROUND_COLOR: Key<Color> = Key::new("instant_desktop.palette.id_background_color");
pub const ID_TEXT_COLOR: Key<Color> = Key::new("instant_desktop.palette.id_text_color");

pub fn add_to_env(env: &mut Env) {
    env.set(PRIMARY, Color::rgb8(0xff, 0xd2, 0x00));
    env.set(LIGHT, Color::rgb8(0x8a, 0x91, 0x99));
    env.set(LIGHT_HOVERED, Color::rgb8(0xbf, 0xc4, 0xca));
    env.set(DARK, Color::rgb8(0x33, 0x31, 0x32));

    env.set(MONITOR_BACKGROUND_COLOR, Color::BLACK);
    env.set(ID_BACKGROUND_COLOR, env.get(LIGHT));
    env.set(ID_TEXT_COLOR, Color::WHITE);
}
