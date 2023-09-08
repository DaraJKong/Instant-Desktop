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

use lazy_regex::{regex, regex_replace_all};
use std::{fs, process::Command};

use crate::config::Config;

pub fn start_rdc_session(config: &Config, selected_monitors: Vec<u32>) {
    // read base file
    let mut rdp_config =
        fs::read_to_string(&config.base_config_path).expect("failed to read base config file");

    // use_multimon parameter
    let use_multimon_r = regex!(r#"^(use\smultimon:i:).*$"#im);

    if use_multimon_r.is_match(&rdp_config) {
        rdp_config = regex_replace_all!(
            r#"^(use\smultimon:i:).*$"#im,
            &rdp_config,
            |_, key| format!("{}1", key),
        )
        .to_string();
    } else {
        rdp_config.push_str("\nuse multimon:i:1");
    }

    // selectedmonitors parameter
    let selected_value = selected_monitors
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let selectedmonitors_r = regex!(r#"^(selectedmonitors:s:).*$"#im);

    if selectedmonitors_r.is_match(&rdp_config) {
        rdp_config = regex_replace_all!(
            r#"^(selectedmonitors:s:).*$"#im,
            &rdp_config,
            |_, key| format!("{}{}", key, selected_value),
        )
        .to_string();
    } else {
        rdp_config.push_str(&format!("\nselectedmonitors:s:{}", selected_value));
    }

    // write custom file
    let custom_rdp_path = config.directories.custom_rdp_path();

    if fs::write(&custom_rdp_path, rdp_config).is_ok() {
        // start remote desktop connection
        Command::new("mstsc")
            .args(["/edit", custom_rdp_path.to_str().unwrap()])
            .spawn()
            .expect("failed to execute process");
    } else {
        panic!(
            "failed to write custom config file: {}",
            custom_rdp_path.to_str().unwrap()
        );
    }
}
