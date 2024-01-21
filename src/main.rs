#![windows_subsystem = "windows"]

extern crate user32;
extern crate winapi;

use std::{process::Command, path::PathBuf};

use config::get_keymaps;
use serde_json::Value;
use tray_item::TrayItem;
use winapi::{shared::windef::HHOOK__, um::winuser};
mod config;
mod parse;

const APP_NAME: &str = "Keybind Actions";
const WH_KEYBOARD_LL: i32 = 13;
const WM_KEYDOWN: usize = 0x100;
const WM_KEYUP: usize = 0x101;

trait Action {
    fn run(&self);
}

struct KeybindAction {
    keys: Vec<u32>,
    command: String,
}

impl Action for KeybindAction {
    fn run(&self) {
        Command::new("cmd")
            .args(["/C", &self.command])
            .spawn()
            .expect("failed to start");
    }
}

enum Message {
    OpenConfig,
    Reload,
    Quit,
}

static mut KEY_SEQUENCE: Vec<u32> = Vec::new();
static mut KEYBIND_ACTIONS: Vec<KeybindAction> = Vec::new();
static mut HOOK_ID: Option<*mut HHOOK__> = None;

fn main() {
    let keymaps_path = get_keymaps().expect("Failed to get keymaps");
    
    unsafe {
        let mut task = run_keyboard_loop(&keymaps_path);
        let (tx, rx) = std::sync::mpsc::sync_channel(1);

        let mut tray = TrayItem::new(APP_NAME, tray_item::IconSource::Resource("tray-default")).expect("Failed to create tray item");

        let edit_tx = tx.clone();
        tray.add_menu_item("Open config folder", move || {
            edit_tx.send(Message::OpenConfig).unwrap();
        }).unwrap();

        let reload_tx = tx.clone();
        tray.add_menu_item("Reload", move || {
            reload_tx.send(Message::Reload).unwrap();
        }).unwrap();

        let quit_tx = tx.clone();
        tray.add_menu_item("Exit", move || {
            quit_tx.send(Message::Quit).unwrap();
        }).unwrap();

        loop {
            match rx.recv() {
                Ok(Message::OpenConfig) => {
                    let keymaps = get_keymaps().expect("Failed to get keymaps file");
                    let parent_dir = keymaps.parent().unwrap();
                    let parent_dir = parent_dir.to_str().unwrap();
                    
                    Command::new("explorer")
                        .arg(parent_dir)
                        .spawn()
                        .unwrap();
                },
                Ok(Message::Reload) => {
                    winuser::UnhookWindowsHookEx(HOOK_ID.take().unwrap());
                    drop(task);

                    task = run_keyboard_loop(&keymaps_path);
                },
                Ok(Message::Quit) => {
                    winuser::UnhookWindowsHookEx(HOOK_ID.take().unwrap());
                    std::process::exit(0x0100);
                },
                _ => {}
            }
        }
    }
}

unsafe fn run_keyboard_loop(keymaps_path: &PathBuf) -> async_std::task::JoinHandle<()> {
   KEYBIND_ACTIONS = load_keybind_actions(keymaps_path);
   async_std::task::spawn(keyboard_loop())
}

fn load_keybind_actions(path: &PathBuf) -> Vec<KeybindAction> {
    let contents = std::fs::read_to_string(path).expect("Failed to read file");
    let value: Vec<Value> = serde_json::from_str(&contents).expect("Failed to parse file"); 
    
    value.iter()
        .map(|v| {parse_keymap(&v.clone())})
        .collect()
}

fn parse_keymap(value: &Value) -> KeybindAction {
    let keys: String = value["keys"].to_string().replace("\"", "");
    let command: String = value["command"].to_string().replace("\"", "");

    let keys = parse::parse_key_combo(&keys);
    KeybindAction { keys, command }
}

async unsafe fn keyboard_loop() {
    HOOK_ID = Some(winuser::SetWindowsHookExW(
        WH_KEYBOARD_LL,
        Some(hook_callback),
        std::ptr::null_mut(),
        0,
    )); 

    let mut msg: winuser::MSG = std::mem::zeroed();

    loop {
        if winuser::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) == 0 {
            break;
        }
    }
}

unsafe extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    if wparam == WM_KEYDOWN { 
        append_key_sequence(lparam);
       
        if let Some(keybind_action) = find_keybind_action(&KEY_SEQUENCE) {
            keybind_action.run();
        }
    } else if wparam == WM_KEYUP {
        KEY_SEQUENCE.clear();
    }

    winuser::CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam)
}

unsafe fn append_key_sequence(lparam: isize) {
    let kbd_struct = lparam as *const winuser::KBDLLHOOKSTRUCT;
    let virtual_key = (*kbd_struct).vkCode;

    if !KEY_SEQUENCE.contains(&virtual_key) {
        KEY_SEQUENCE.push(virtual_key);
    }
}

unsafe fn find_keybind_action(key_seq: &Vec<u32>) -> Option<&KeybindAction> {
    KEYBIND_ACTIONS
        .iter()
        .find(|action| key_seq == &action.keys)
}
