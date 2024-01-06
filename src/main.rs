use std::mem::size_of;
use std::process::Command;

use windows::core::s;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::MONITOR_DEFAULTTONEAREST;
use windows::Win32::Graphics::Gdi::MonitorFromPoint;
use windows::Win32::Graphics::Gdi::MONITORINFO;
use windows::Win32::Graphics::Gdi::GetMonitorInfoA;
use windows::Win32::Graphics::Gdi::MONITORINFOEXA;
use windows::Win32::Graphics::Gdi::MapWindowPoints;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Foundation::POINT;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::UI::WindowsAndMessaging::SetParent;
use windows::Win32::UI::WindowsAndMessaging::FindWindowExA;
use windows::Win32::UI::WindowsAndMessaging::EnumWindows;
use windows::Win32::UI::WindowsAndMessaging::SendMessageA;
use windows::Win32::UI::WindowsAndMessaging::SET_WINDOW_POS_FLAGS;
use windows::Win32::UI::WindowsAndMessaging::SetWindowPos;
use windows::Win32::UI::WindowsAndMessaging::FindWindowA;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;
fn main() {
    let pattern = std::env::args().nth(1).expect("no file to play given ");
    let args = vec!["--player-operation-mode=pseudo-gui", "--force-window=yes", "--fs", "--terminal=no", "--no-audio", "--loop=inf", pattern.as_str()];

    let output = Command::new("mpv")
        .args(&args)
        .spawn();

    match output {
        Ok(_) => println!("mpv started with specified arguments"),
        Err(e) => eprintln!("Failed to start mpv: {}", e),
    }


    let window: HWND = get_window();
    println!("windo {:#?}",window);

    wp_fullscreen(window);

    let progman = unsafe { FindWindowA(s!("Progman"), None) };
    /*
    * this is basically all the magic. it's an undocumented window message that
    * forces windows to spawn a window with class "WorkerW" behind deskicons
    */
    unsafe{
        let message0 =         SendMessageA( progman,
            0x52c,
            WPARAM(0),
            LPARAM(0));

        println!("message0 {:#?}", message0);
        let message1 =         SendMessageA( progman,
            0x52c,
            WPARAM(0),
            LPARAM(1));
        println!("message1 {:#?}", message1);
    }

    println!("progman {:#?}", progman);
    let mut worker_w = 0 as isize;
    println!("worker_w {:#?}",worker_w  );

    unsafe {
        let result =EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut worker_w as *mut isize as isize),
        );
        println!("result enum windows: {:#?}", result)
    }
    println!("worker_w {:#?}",worker_w  );

    let parent_res  =   unsafe {
        SetParent(window, HWND(worker_w))
    };
    println!("parent_res  {:#?}",parent_res  );
}

fn get_window() -> HWND {
    let window =     unsafe{
        FindWindowA(s!("mpv"), None)
    };
    println!("get window: {:#?}", window);
    match window {
        HWND(0) => return get_window(),
        _ => return window, 
    }
}

fn wp_fullscreen(window: HWND) {
    let mut rect = unsafe { std::mem::zeroed() };
    let result = unsafe { GetWindowRect(window, &mut rect) };
    println!(" GetWindowRect {:#?}",result );
    let point = 
    POINT {x: rect.top, y: rect.left}; 
    let result  = unsafe{
        MonitorFromPoint(point , MONITOR_DEFAULTTONEAREST)
    };
    println!("MonitorFromRect {:#?}",result );
    let mon = unsafe{ MonitorFromPoint(POINT{x: rect.left, y: rect.top}, MONITOR_DEFAULTTONEAREST)};
    let info = MONITORINFOEXA {
        monitorInfo: MONITORINFO {
            cbSize: size_of::<MONITORINFOEXA>() as u32,
            ..Default::default()
        },
        szDevice: [0; 32],
    };
    let result = unsafe{
        GetMonitorInfoA(mon, &info as *const _ as *mut _)
    };
    println!("GetMonitorInfoA: {:#?}",result );
    println!("GetTest: {:#?}",info.monitorInfo );
    let result =  unsafe {
        MapWindowPoints(None, window, &mut [point] )
    }; 
    println!("MapWindowPoints: {:#?}",result );
    wp_move(window, info.monitorInfo.rcMonitor.left, info.monitorInfo.rcMonitor.top, info.monitorInfo.rcMonitor.right, info.monitorInfo.rcMonitor.bottom)

}


extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let shell = unsafe { FindWindowExA(hwnd, None, s!("SHELLDLL_DefView"), None) };

    if is_valid_hwnd(&shell) {
        let worker_w = unsafe { FindWindowExA(None, hwnd, s!("WorkerW"), None) };

        if is_valid_hwnd(&worker_w) {
            unsafe {
                *(lparam.0 as *mut isize) = worker_w.0;
            }
        }
    }

    BOOL(1)
}

fn is_valid_hwnd(hwnd: &HWND) -> bool {
    hwnd.0 != 0
}

fn wp_move(window: HWND, left: i32, top: i32, right: i32, bottom: i32) {
    let flags = 0 ;
    let res = unsafe{
        SetWindowPos(window, HWND(0), left, top, right, bottom, SET_WINDOW_POS_FLAGS(flags),)
    };
    println!("wp move result {:#?}", res);
}


