use tauri::{Runtime, WebviewWindow};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW, SetWindowLongW, SetWindowPos, 
    GWL_EXSTYLE, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, 
    SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE, HWND_TOPMOST
};

pub fn set_input_window_style<R: Runtime>(window: &WebviewWindow<R>) {
    let hwnd = window.hwnd().unwrap().0;
    let hwnd = HWND(hwnd as _);

    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);

        let new_ex_style = ex_style | (WS_EX_NOACTIVATE.0 as i32) | (WS_EX_TOOLWINDOW.0 as i32);
        
        SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);

        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE 
        );
    }
}