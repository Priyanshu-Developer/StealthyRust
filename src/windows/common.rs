use system32::EventType;
use crate::windows::keyboard::Keyboard;
use crate::windows::keycodes::key_from_code;
use lazy_static::lazy_static;
use std::convert::TryInto;
use std::os::raw::c_int;
use std::ptr::null_mut;
use std::sync::Mutex;
use winapi::shared::minwindef::{DWORD, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::{SetWindowsHookExA, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP};
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;

pub static mut HOOK: HHOOK = null_mut();
lazy_static! {
    pub(crate) static ref KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard::new().unwrap());
}

pub unsafe fn get_code(lpdata: LPARAM) -> DWORD {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);
    kb.vkCode
}
pub unsafe fn get_scan_code(lpdata: LPARAM) -> DWORD {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);
    kb.scanCode
}

pub unsafe fn convert(param: WPARAM, lpdata: LPARAM) -> Option<EventType> {
    match param.try_into() {
        Ok(WM_KEYDOWN) | Ok(WM_SYSKEYDOWN) => {
            let code = get_code(lpdata);
            let key = key_from_code(code as u16);
            Some(EventType::KeyPress(key))
        }
        Ok(WM_KEYUP) | Ok(WM_SYSKEYUP) => {
            let code = get_code(lpdata);
            let key = key_from_code(code as u16);
            Some(EventType::KeyRelease(key))
        }
        _ => None,
    }
}

type RawCallback = unsafe extern "system" fn(code: c_int, param: WPARAM, lpdata: LPARAM) -> LRESULT;
pub enum HookError {
    Key(DWORD),
}

pub unsafe fn set_key_hook(callback: RawCallback) -> Result<(), HookError> {
    let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), null_mut(), 0);

    if hook.is_null() {
        let error = GetLastError();
        return Err(HookError::Key(error));
    }
    HOOK = hook;
    Ok(())
}
