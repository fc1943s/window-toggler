use pathsearch::find_executable_in_path;
use std::{fs, io, mem};
use hotkey::modifiers::{ALT, SHIFT};
use windows_win::raw::window;
use winapi::shared::windef::{HWND, HWND__, POINT};
use winapi::um::winuser;
use winapi::um::winuser::{SC_MINIMIZE, SC_RESTORE, SendInput, INPUT_KEYBOARD};
use winapi::um::winnt::LONG;
use std::mem::size_of;


#[derive(serde::Deserialize)]
pub struct Config {
    window_title: String
}

unsafe fn get_foreground_window() -> *mut HWND__ {
    winuser::GetForegroundWindow()
}

unsafe fn set_foreground_window(hwnd: HWND) {
    winuser::SetForegroundWindow(hwnd);
}

unsafe fn window_from_point(x: LONG, y: LONG) -> *mut HWND__ {
    winuser::WindowFromPoint(POINT { x, y })
}

unsafe fn _send_key_event(vk: u16, flags: u32) {
    let key_input = winuser::KEYBDINPUT {
        wVk: vk,
        wScan: 0,
        dwFlags: flags,
        time: 0,
        dwExtraInfo: 0,
    };

    let mut input = winuser::INPUT {
        type_: INPUT_KEYBOARD,
        u: mem::zeroed(),
    };
    *input.u.ki_mut() = key_input;

    SendInput(1, &mut input, size_of::<winuser::INPUT>() as i32);
}

unsafe fn toggle(window_title: &String) -> Result<(), io::Error> {
    let foreground = get_foreground_window();
    let search_result = window::get_by_title(window_title, None)?;

    println!("\npress");
    println!("foreground: {:?}", window::get_text(foreground));
    println!("search_result: {:?}", search_result.iter().map(|&hwnd| window::get_text(hwnd)).collect::<Vec<_>>());

    match search_result.as_slice() {
        [search_result_hwnd] if *search_result_hwnd == foreground => {
            window::send_sys_command(*search_result_hwnd, SC_MINIMIZE, 0);
            println!("1:minimized result");

            let new_foreground = window_from_point(200, 200);
            set_foreground_window(new_foreground);
            println!("new_foreground: {:?}", window::get_text(new_foreground));
        }
        [search_result_hwnd] => {


            // get window state
            // winuser::ShowWindow(*search_result_hwnd, SW_RESTORE);

            // let mut windowplacement = winuser::WINDOWPLACEMENT {
            //     length: 0,
            //     flags: 0,
            //     showCmd: 0,
            //     ptMinPosition: POINT {},
            //     ptMaxPosition: POINT {},
            //     rcNormalPosition: RECT {}
            // };
            let mut result_placement: winuser::WINDOWPLACEMENT = mem::zeroed();
            winuser::GetWindowPlacement(*search_result_hwnd, &mut result_placement);
            println!("placement_flags: {:?}", result_placement.flags);
            println!("placement_length: {:?}", result_placement.length);
            println!("placement_ptMaxPosition: {:?}", result_placement.ptMaxPosition.x);
            println!("placement_ptMinPosition: {:?}", result_placement.ptMinPosition.x);
            println!("placement_rcNormalPosition: {:?}", result_placement.rcNormalPosition.top);
            println!("placement_showCmd: {:?}", result_placement.showCmd);

            // window::send_message(*search_result_hwnd, WM_KEYUP, VK_MENU as usize, 0, None)?;

            window::send_sys_command(*search_result_hwnd, SC_RESTORE, 0);

            let new_foreground = get_foreground_window();
            println!("new_foreground: {:?}", window::get_text(new_foreground));

            if foreground == new_foreground {
                println!("equal1");
            }

            if new_foreground != *search_result_hwnd {
                set_foreground_window(*search_result_hwnd);

                println!("new_foreground2: {:?}", window::get_text(*search_result_hwnd));
            }

            // send_key_event(VK_ESCAPE as u16, 0);
            // sleep(Duration::from_millis(500));
            // send_key_event(VK_ESCAPE as u16, 0);
            // send_key_event(VK_ESCAPE as u16, 0);
        }
        _ => ()
    }

    Ok(())
}

fn key_pressed(window_title: &String) {
    unsafe {
        toggle(window_title).unwrap();
    }
}

fn main() {
    let config_path = find_executable_in_path("window-toggler.toml").expect("Config file not found");
    let config_bytes = fs::read(config_path).expect("Error reading config file");
    let config: Config = toml::from_slice(&config_bytes).expect("Error parsing config file");

    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(ALT | SHIFT, 'W' as u32, move || key_pressed(&config.window_title)).unwrap();
    println!("Listening...");
    hk.listen();
}

