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

/*use druid::widget::{AspectRatioBox, Button, Flex, Label, LineBreaking};
use druid::{AppLauncher, Color, Widget, WidgetExt, WindowDesc};*/

use instant_desktop::windows::Monitors;

/*fn build_app() -> impl Widget<u32> {
    // Usually we put all the widgets in one big tree using builder-style
    // methods. Sometimes we split them up in declarations to increase
    // readability. In this case we also have some recurring elements,
    // we add those in a loop later on.
    let mut col = Flex::column().with_child(
        // The `Flex`'s first child is another Flex! In this case it is
        // a row.
        Flex::row()
            // The row has its own children.
            .with_child(
                Label::new("One")
                    .fix_width(60.0)
                    .background(Color::rgb8(0x77, 0x77, 0))
                    .border(Color::WHITE, 3.0)
                    .center(),
            )
            // Spacing element that will fill all available space in
            // between label and a button. Notice that weight is non-zero.
            // We could have achieved a similar result with expanding the
            // width and setting the main-axis-allignment to SpaceBetween.
            .with_flex_spacer(1.0)
            .with_child(Button::new("Two").padding(20.))
            // After we added all the children, we can set some more
            // values using builder-style methods. Since these methods
            // dont return the original `Flex` but a SizedBox and Container
            // respectively, we have to put these at the end.
            .fix_height(100.0)
            //turquoise
            .background(Color::rgb8(0, 0x77, 0x88)),
    );

    for i in 0..5 {
        // Give a larger weight to one of the buttons for it to
        // occupy more space.
        let weight = if i == 2 { 3.0 } else { 1.0 };
        // call `expand_height` to force the buttons to use all their provided flex
        col.add_flex_child(Button::new(format!("Button #{i}")).expand_height(), weight);
    }

    // aspect ratio box
    let aspect_ratio_label = Label::new("This is an aspect-ratio box. Notice how the text will overflow if the box becomes too small.")
        .with_text_color(Color::BLACK)
        .with_line_break_mode(LineBreaking::WordWrap)
        .center();
    let aspect_ratio_box = AspectRatioBox::new(aspect_ratio_label, 4.0)
        .border(Color::BLACK, 1.0)
        .background(Color::WHITE);
    col.add_flex_child(aspect_ratio_box.center(), 1.0);

    // This method asks Druid to draw colored rectangles around our widgets,
    // so we can visually inspect their layout rectangles.
    col.debug_paint_layout()
}*/

fn main() {
    let active_monitors_list = Monitors::enum_active();

    active_monitors_list.display_list();

    /*let window = WindowDesc::new(build_app()).title("Very flexible");

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(0)
        .expect("launch failed");*/
}
