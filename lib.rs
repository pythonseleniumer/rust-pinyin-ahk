use pinyin::{ToPinyin, ToPinyinMulti};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn pinyin_plain(text: *const c_char) -> *mut c_char {
    if text.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(text) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let result: Vec<String> = input
        .to_pinyin()
        .flatten()
        .map(|p| p.plain().to_string())
        .collect();
    let output = result.join(" ");
    let c_string = CString::new(output).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn pinyin_with_tone(text: *const c_char) -> *mut c_char {
    if text.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(text) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let result: Vec<String> = input
        .to_pinyin()
        .flatten()
        .map(|p| p.with_tone().to_string())
        .collect();
    let output = result.join(" ");
    let c_string = CString::new(output).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn pinyin_with_tone_num(text: *const c_char) -> *mut c_char {
    if text.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(text) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let result: Vec<String> = input
        .to_pinyin()
        .flatten()
        .map(|p| p.with_tone_num().to_string())
        .collect();
    let output = result.join(" ");
    let c_string = CString::new(output).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn pinyin_with_tone_num_end(text: *const c_char) -> *mut c_char {
    if text.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(text) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let result: Vec<String> = input
        .to_pinyin()
        .flatten()
        .map(|p| p.with_tone_num_end().to_string())
        .collect();
    let output = result.join(" ");
    let c_string = CString::new(output).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
