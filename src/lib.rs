use pinyin::{ToPinyin, ToPinyinMulti};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// 将汉字转换为无声调拼音
/// 返回指针，调用者需要手动释放
#[no_mangle]
pub extern "C" fn pinyin_plain(text: *const c_char) -> *mut c_char {
    if text.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(text) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Failed to convert C string to UTF-8");
            return std::ptr::null_mut();
        }
    };

    let result: Vec<String> = input
        .chars()
        .filter_map(|ch| {
            ch.to_pinyin().map(|p| p.plain().to_string())
        })
        .collect();

    if result.is_empty() {
        return std::ptr::null_mut();
    }

    let output_str = result.join(" ");
    match CString::new(output_str) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// 将汉字转换为带声调的拼音
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
        .chars()
        .filter_map(|ch| {
            ch.to_pinyin().map(|p| p.with_tone().to_string())
        })
        .collect();

    if result.is_empty() {
        return std::ptr::null_mut();
    }

    let output_str = result.join(" ");
    match CString::new(output_str) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// 将汉字转换为数字标注声调的拼音
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
        .chars()
        .filter_map(|ch| {
            ch.to_pinyin().map(|p| p.with_tone_num().to_string())
        })
        .collect();

    if result.is_empty() {
        return std::ptr::null_mut();
    }

    let output_str = result.join(" ");
    match CString::new(output_str) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// 将汉字转换为数字在末尾的拼音
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
        .chars()
        .filter_map(|ch| {
            ch.to_pinyin().map(|p| p.with_tone_num_end().to_string())
        })
        .collect();

    if result.is_empty() {
        return std::ptr::null_mut();
    }

    let output_str = result.join(" ");
    match CString::new(output_str) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// 释放字符串内存
#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
