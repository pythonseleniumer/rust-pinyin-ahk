use pinyin::{ToPinyin, ToPinyinMulti};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// 将汉字转换为无声调拼音，结果写入缓冲区
/// 例如: "中国人" -> "zhong guo ren"
#[no_mangle]
pub extern "C" fn pinyin_plain(text: *const c_char, output: *mut u8, output_len: usize) -> usize {
    if text.is_null() || output.is_null() {
        return 0;
    }

    let c_str = unsafe { CStr::from_ptr(text) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let result: Vec<String> = input
        .to_pinyin()
        .flatten()
        .map(|p| p.plain().to_string())
        .collect();

    let output_str = result.join(" ");
    let bytes = output_str.as_bytes();

    if bytes.len() >= output_len {
        return 0;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), output, bytes.len());
        *output.add(bytes.len()) = 0;
    }

    bytes.len()
}

/// 将汉字转换为带声调的拼音，结果写入缓冲区
/// 例如: "中国人" -> "zhōng guó rén"
#[no_mangle]
pub extern "C" fn pinyin_with_tone(text: *const c_char, output: *mut u8, output_len: usize) -> usize {
    if text.is_null() || output.is_null() {
        return 0;
    }

    let c_str = unsafe { CStr::from_ptr(text) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let result: Vec<String> = input
        .to_pinyin()
        .flatten()
        .map(|p| p.with_tone().to_string())
        .collect();

    let output_str = result.join(" ");
    let bytes = output_str.as_bytes();

    if bytes.len() >= output_len {
        return 0;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), output, bytes.len());
        *output.add(bytes.len()) = 0;
    }

    bytes.len()
}

/// 将汉字转换为数字标注声调的拼音，结果写入缓冲区
/// 例如: "中国人" -> "zho1ng guo2 re2n"
#[no_mangle]
pub extern "C" fn pinyin_with_tone_num(text: *const c_char, output: *mut u8, output_len: usize) -> usize {
    if text.is_null() || output.is_null() {
        return 0;
    }

    let c_str = unsafe { CStr::from_ptr(text) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let result: Vec<String> = input
        .to_pinyin()
        .flatten()
        .map(|p| p.with_tone_num().to_string())
        .collect();

    let output_str = result.join(" ");
    let bytes = output_str.as_bytes();

    if bytes.len() >= output_len {
        return 0;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), output, bytes.len());
        *output.add(bytes.len()) = 0;
    }

    bytes.len()
}

/// 将汉字转换为数字在末尾的拼音，结果写入缓冲区
/// 例如: "中国人" -> "zhong1 guo2 ren2"
#[no_mangle]
pub extern "C" fn pinyin_with_tone_num_end(text: *const c_char, output: *mut u8, output_len: usize) -> usize {
    if text.is_null() || output.is_null() {
        return 0;
    }

    let c_str = unsafe { CStr::from_ptr(text) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let result: Vec<String> = input
        .to_pinyin()
        .flatten()
        .map(|p| p.with_tone_num_end().to_string())
        .collect();

    let output_str = result.join(" ");
    let bytes = output_str.as_bytes();

    if bytes.len() >= output_len {
        return 0;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), output, bytes.len());
        *output.add(bytes.len()) = 0;
    }

    bytes.len()
}
