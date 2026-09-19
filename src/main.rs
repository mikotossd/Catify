#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
#![allow(static_mut_refs)]

use windows::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_UNICODE, SendInput, VIRTUAL_KEY,
    VK_RETURN,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, HHOOK, KBDLLHOOKSTRUCT, LLKHF_INJECTED, MSG,
    SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN,
};
use windows::core::w;

const MUTEX_NAME: windows::core::PCWSTR = w!("Global\\catify-mutex");

static mut G_HOOK: Option<HHOOK> = None;

unsafe fn send_unicode(s: &str) {
    let mut inputs: Vec<INPUT> = Vec::new();
    for ch in s.chars() {
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: ch as u16,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        });
    }
    let _ = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
}

unsafe extern "system" fn ll_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let info = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        if !info.flags.contains(LLKHF_INJECTED) {
            if wparam.0 as u32 == WM_KEYDOWN && info.vkCode == VK_RETURN.0 as u32 {
                send_unicode("喵");
            }
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

fn main() {
    let _mutex = match unsafe { CreateMutexW(None, false, MUTEX_NAME) } {
        Ok(h) => h,
        Err(_) => return,
    };
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        return;
    }

    unsafe {
        if let Ok(h) = SetWindowsHookExW(WH_KEYBOARD_LL, Some(ll_hook), None, 0) {
            G_HOOK = Some(h);
        } else {
            return;
        }
    }
    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    unsafe {
        if let Some(h) = G_HOOK.take() {
            let _ = UnhookWindowsHookEx(h);
        }
    }
}
