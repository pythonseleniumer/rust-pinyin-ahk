# rust-pinyin-ahk
Rust pinyin library wrapper for AutoHotkey v2
#
#
#Requires AutoHotkey v2.0

dllPath := A_ScriptDir . "\rust_pinyin_ahk.dll"

; 测试
text := "中国人"

; 无声调
result := DllCall(dllPath . "\pinyin_plain", "AStr", text, "AStr")
MsgBox(result)  ; 输出: zhong guo ren

; 带声调
result := DllCall(dllPath . "\pinyin_with_tone", "AStr", text, "AStr")
MsgBox(result)  ; 输出: zhōng guó rén

; 数字声调
result := DllCall(dllPath . "\pinyin_with_tone_num", "AStr", text, "AStr")
MsgBox(result)  ; 输出: zho1ng guo2 re2n

; 末尾数字
result := DllCall(dllPath . "\pinyin_with_tone_num_end", "AStr", text, "AStr")
MsgBox(result)  ; 输出: zhong1 guo2 ren2
