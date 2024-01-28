# Keybind Actions
## Setup
1. Install the .exe file from the [Releases](https://github.com/Zenqrt/Keybind-Actions/releases) page.
2. Run the .exe file and wait for the tray icon to appear.
3. Click on the tray icon and click `Open config folder`.
4. Open `keymaps.json`.
5. To add a keybind, add the following code inside the brackets and replace as necessary. Keys should follow the format: `key+key`. Seek the list of keys [here](https://youtu.be/dQw4w9WgXcQ?si=i1ebDg1UZCLpuv9V).
```json
{
    "keys": "keys_go_here",
    "command": "command_goes_here"
}
```
6. Save the file.
7. Click on the tray icon and click `Reload`.
8. Use the keybind and it will run the command.

## TODO
* [ ] Make better keybind detection
* [ ] Add macOS support
