# Instant Desktop
This application is in development. It is meant to replace [Infinity Desktop][infurl].

Instant Desktop doesn't use "magic" workarounds to get the monitor IDs, but instead uses the Windows API through the [official crate][wincrate].

#### Table of Contents

1. [Features](#features)
2. [Installation](#installation)
3. [How To Use](#how-to-use)
   - [Settings](#settings)
4. [TODO](#todo)
4. [License](#license)
4. [Contribution](#contribution)

## Features

- Provides fast and easy selection of multiple monitors when using RDC
- Allows customization of essential settings through a configuration file
- Native application with rapid startup and small binary size
- Easy installation

## Installation

1. Download the executable, preferably from the latest release:

   | Version                  | Operating System | Download URL          |
   | ------------------------ | ---------------- | --------------------- |
   | [0.1.0][v0.1.0] (latest) | Windows          | [Download][v0.1.0url] |

2. Make sure you have [Remote Desktop Connection][rdc] installed and follow the steps to enable the feature on Windows. This step can be skipped in most cases as it's installed by default.

3. Open the executable you downloaded.

4. Optionally, you can create a desktop shortcut and pin it to the taskbar.

# How To Use

To connect to your remote desktop using multiple monitors, double-click on the executable file to open it. Numbers will appear on each monitor. Those represent the monitors' IDs.

Left-click on any screen to select or unselect it. A yellow background means the monitor is selected for the remote connection. Monitors you don't select will be used for your current computer. Once you are satisfied with your setup, simply press the Enter key. The screens will go back to normal as the remote session is starting using a custom RDP file.

To cancel the remote connection, you can press the Escape key, the Delete key or the Backspace key.

The app will create a folder named "Instant-Desktop" in the directory "C:\\Users\\{USERNAME}\\AppData\\Roaming\\." You will find a useful configuration file in it where settings are stored. Replace "{USERNAME}" with your actual username to find the right folder, because the path varies depending on who is using the computer.

## Settings

To consult or modify Instant Desktop's parameters, open the configuration file "C:\\Users\\{USERNAME}\\AppData\\Roaming\\Instant-Desktop\\config.ini" in any text editor. To quickly navigate to the folder, you can enter "%appdata%" in the input field at the top of the Windows Explorer. This shortcut will bring you directly to "C:\\Users\\{USERNAME}\\AppData\\Roaming\\." Otherwise, simply replace "{USERNAME}" with your username in the path.

#### **base_config_path**

> Sets the path to the base configuration file that will be used when starting the remote session. For it to work, you need to use an absolute path.

#### **fullscreen**

> Sets whether the screen overlays are displayed in fullscreen or not in the app. Set to "true" for the app to be in fullscreen mode or "false" to show the taskbar while selecting the screens.

#### **edit_connection**

> If set to "true", the remote connection will enable you to edit the settings before proceding. Set to "false" if you want to skip that step and save time. It is recommended to set the [base_config_path](#base_config_path) setting before disabling this setting, because it ensures you always connect with the right configuration.

# TODO

- [ ] Enable user to change settings easily
- [ ] How to use tips
- [ ] Shortcuts reminder
- [ ] Remember last monitors selection
- [ ] Add a new icon
- [ ] Customizable theme
- [ ] Notify user when selected monitors are of different resolutions and might not give expected results
- [ ] Create automatic installer and uninstaller
- [ ] Workaround to use monitors of different resolutions

## License

Licensed under

 * Apache License, Version 2.0
   ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[infurl]: https://github.com/DaraJKong/Infinity-Desktop
[wincrate]: https://crates.io/crates/windows
[v0.1.0]: https://github.com/DaraJKong/Instant-Desktop/releases/tag/v0.1.0
[v0.1.0url]: https://github.com/DaraJKong/Instant-Desktop/releases/download/v0.1.0/instant-desktop.exe