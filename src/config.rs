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

use std::{fs, path::PathBuf};

use directories::{ProjectDirs, UserDirs};
use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Data, Clone)]
pub struct Config {
    #[serde(skip)]
    #[data(ignore)]
    pub directories: Directories,
    #[data(same_fn = "PartialEq::eq")]
    pub base_config_path: PathBuf,
    pub fullscreen: bool,
    pub edit_connection: bool,
}

impl Default for Config {
    fn default() -> Self {
        let directories = Directories::default();

        let mut base_config_path = directories.document_dir();
        base_config_path.push("Default.rdp");

        let config_dir = directories.project.config_dir().to_path_buf();
        let data_dir = directories.project.data_dir().to_path_buf();
        fs::create_dir_all(&config_dir).expect("failed to create config directory");
        fs::create_dir_all(&data_dir).expect("failed to create data directory");

        Self {
            directories,
            base_config_path,
            fullscreen: true,
            edit_connection: true,
        }
    }
}

impl Config {
    pub fn save(&self) {
        let config_string = serde_yaml::to_string(self).expect("failed to serialize");
        fs::write(self.directories.config_path(), config_string)
            .expect("failed to write config file");
    }

    pub fn load(&mut self) {
        if let Ok(config_content) = fs::read_to_string(self.directories.config_path()) {
            *self = serde_yaml::from_str(&config_content).expect("failed to deserialize");
        } else {
            self.save();
        }
    }
}

#[derive(Clone)]
pub struct Directories {
    project: ProjectDirs,
    user: UserDirs,
}

impl Default for Directories {
    fn default() -> Self {
        let project = ProjectDirs::from("", "", "Instant-Desktop")
            .expect("failed to get project directories");
        let user = UserDirs::new().expect("failed to get user directories");

        Self { project, user }
    }
}

impl Directories {
    pub fn config_path(&self) -> PathBuf {
        let mut config_path = self.project.config_dir().to_path_buf();
        config_path.push("config.yaml");

        config_path
    }

    pub fn document_dir(&self) -> PathBuf {
        self.user.document_dir().unwrap().to_path_buf()
    }

    pub fn custom_rdp_path(&self) -> PathBuf {
        let mut custom_rdp_path = self.project.data_dir().to_path_buf();
        custom_rdp_path.push("custom.rdp");

        custom_rdp_path
    }
}
