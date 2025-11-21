pub type Wchar = u16;
use core::ffi::{CStr, c_char};

macro_rules! CL {
    ($c: literal) => {
        $c as u16
    };
}

pub fn str_null_checks(s: &[u16]) -> Result<(), ToIntErr> {
    if s.is_empty() {
        Err(ToIntErr::FoundNullPtr)
    } else if s[0] == CL!('\0') {
        Err(ToIntErr::EndOfStr)
    } else {
        Ok(())
    }
}

pub fn is_end_of_num(s: &[Wchar], offset: usize, base: u16) -> bool {
    offset >= s.len()
        || s[offset] < CL!('0')
        || (base < 10 && s[offset] >= base + CL!('0'))
        || (base >= 10
            && ((s[offset] > CL!('9') && s[offset] < CL!('A'))
                || (s[offset] > CL!('F') && s[offset] < CL!('a'))
                || s[offset] > CL!('f')))
}

pub fn is_digit(s: &[Wchar], offset: usize, base: u16) -> bool {
    s[offset] >= CL!('0') && s[offset] <= if base < 10 { base + CL!('0') } else { CL!('9') }
}

pub fn is_hex_lower(s: &[Wchar], offset: usize, base: u16) -> bool {
    base == 16 && s[offset] >= CL!('a') && s[offset] <= CL!('f')
}

pub fn is_hex_upper(s: &[Wchar], offset: usize, base: u16) -> bool {
    base == 16 && s[offset] >= CL!('A') && s[offset] <= CL!('F')
}

pub fn check_sign(s: &[u16]) -> (i64, usize) {
    if s[0] == CL!('-') { (-1, 1) } else { (1, 0) }
}

#[derive(Debug)]
pub enum ToIntErr {
    FoundNullPtr,
    EndOfStr,
    InvalidStart,
    InvalidInput,
    LargerThanI32,
}

pub fn strtol(s: &[Wchar], base: u16) -> Result<i64, ToIntErr> {
    str_null_checks(s)?;

    let mut val = 0i64;
    let (sign, offset) = check_sign(s);

    if is_end_of_num(s, offset, base) {
        return Err(ToIntErr::InvalidStart);
    }

    for (i, _) in s.iter().enumerate() {
        if i + offset >= s.len() {
            break;
        }
        if s[i + offset] == CL!('\0') {
            break;
        }
        val *= base as i64;
        let offset = i + offset;
        if is_digit(s, offset, base) {
            val += (s[offset] - CL!('0')) as i64;
        } else if is_hex_upper(s, offset, base) {
            val += (s[offset] - CL!('A') + 10) as i64;
        } else if is_hex_lower(s, offset, base) {
            val += (s[offset] - CL!('a') + 10) as i64;
        } else {
            return Err(ToIntErr::InvalidInput);
        }
    }

    Ok(val * sign)
}

pub fn atoi(s: &[Wchar]) -> Result<i32, ToIntErr> {
    let num = atol(s)?;
    if num >= i32::MIN as i64 && num <= i32::MAX as i64 {
        Ok(num as i32)
    } else {
        Err(ToIntErr::LargerThanI32)
    }
}

pub unsafe fn utf16_cstr_len(ptr: *const u16) -> usize {
    let mut len = 0;
    let mut p = ptr;

    unsafe {
        while *p != 0 {
            len += 1;
            p = p.add(1);
        }
    }

    len
}

pub unsafe fn utf16_cstr_slice<'a>(ptr: *const u16) -> &'a [u16] {
    unsafe {
        let len = utf16_cstr_len(ptr);
        core::slice::from_raw_parts(ptr, len)
    }
}

pub unsafe fn utf8_cstr_len(ptr: *const u8) -> usize {
    let mut len = 0;
    let mut p = ptr;

    unsafe {
        while *p != 0 {
            len += 1;
            p = p.add(1);
        }
    }

    len
}

pub unsafe fn utf8_cstr_slice<'a>(ptr: *const u8) -> &'a [u8] {
    unsafe {
        let len = utf8_cstr_len(ptr);
        core::slice::from_raw_parts(ptr, len)
    }
}

// pub fn mbtowc(buf: &[u8]) -> Result<(u16, usize), ()> {
pub fn atol(s: &[Wchar]) -> Result<i64, ToIntErr> {
    if s.is_empty() {
        return Err(ToIntErr::FoundNullPtr);
    }

    let (sign, offset) = if s[0] == CL!('-') { (-1, 1) } else { (1, 0) };

    for (i, c) in s[offset..].iter().enumerate() {
        let next_idx = i + offset + 1;

        if c == &CL!('0') && next_idx < s.len() {
            let s1 = s[next_idx];
            if s1 == CL!('x') {
                let num = strtol(&s[next_idx + 1..], 16)?;
                return Ok(num * sign);
            }
            if s1 >= CL!('0') && s1 <= CL!('7') {
                let num = strtol(&s[offset..], 8)?;
                return Ok(num * sign);
            }
        }
    }

    let num = strtol(&s[offset..], 10)?;
    Ok(sign * num)
}

macro_rules! b_and {
    ($a: expr, $b: expr) => {
        ($a as u8 & $b as u8) as i32
    };
}

pub fn mbtowc(buf: &[u8]) -> Result<(u16, usize), ()> {
    if buf.is_empty() {
        return Err(());
    }

    // Try to interpret buf's prefix as valid UTF-8
    let s = core::str::from_utf8(buf).map_err(|_| ())?;

    let mut chars = s.chars();
    let ch = chars.next().ok_or(())?;
    let bytes = ch.len_utf8();

    // Encode into UTF-16 (max 2 u16 units)
    let mut tmp = [0u16; 2];
    let encoded = ch.encode_utf16(&mut tmp);

    // Only accept BMP characters (1 u16)
    if encoded.len() != 1 {
        return Err(());
    }

    Ok((encoded[0], bytes))
}

pub unsafe fn memset_zero<T>() -> T {
    unsafe { core::mem::zeroed() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    macro_rules! cstr_16 {
        [$($n:literal),*] => {
            [$($n as u16),*]
        };
    }

    #[test]
    fn test_decimal_positive_str() {
        let buf = cstr_16!['1', '2', '3', '4', '5', '\0'];

        let res = strtol(&buf, 10).unwrap();
        assert_eq!(res, 12345);
    }

    #[test]
    fn test_negative_decimal() {
        let buf = cstr_16!['-', '9', '8', '7'];

        let res = strtol(&buf, 10).unwrap();
        assert_eq!(res, -987);
    }

    #[test]
    fn test_hex_lowercase_str() {
        let buf = cstr_16!['1', 'a', '3', 'f'];

        let res = strtol(&buf, 16).unwrap();
        assert_eq!(res, 0x1a3f);
    }

    #[test]
    fn test_hex_uppercase_str() {
        let buf = cstr_16!['F', 'F'];

        let res = strtol(&buf, 16).unwrap();
        assert_eq!(res, 255);
    }

    #[test]
    fn test_errs_at_invalid_char() {
        let buf = cstr_16!('1', '2', '3', 'x', 'y', 'z');

        let res = strtol(&buf, 10);
        assert!(res.is_err());
    }

    #[test]
    fn test_invalid_start() {
        let buf = cstr_16!['x', '1', '2', '3'];

        let res = strtol(&buf, 10);
        assert!(res.is_err());
    }

    #[test]
    fn test_null_pointer() {
        assert!(strtol(&[], 10).is_err());
    }

    #[test]
    fn test_end_ptr_null_is_allowed() {
        let buf = cstr_16!['5', '5'];

        let res = strtol(&buf, 10).unwrap();
        assert_eq!(res, 55);
        // Should not write to end_ptr, should not crash
    }

    #[test]
    fn test_decimal_positive() {
        let s = cstr_16!('1', '2', '3', '4', '5');
        let result = atol(&s).unwrap();
        assert_eq!(result, 12345);
    }

    #[test]
    fn test_decimal_negative() {
        let s = cstr_16!('-', '6', '7', '8', '9');
        let result = atol(&s).unwrap();
        assert_eq!(result, -6789);
    }

    #[test]
    fn test_hex_uppercase() {
        let s = cstr_16!('0', 'x', '1', 'A', '3', 'F');
        let result = atol(&s).unwrap();
        assert_eq!(result, 0x1A3F);
    }

    #[test]
    fn test_hex_lowercase() {
        let s = cstr_16!('0', 'x', 'd', 'e', 'a', 'd', 'b', 'e', 'e', 'f');
        let result = atol(&s).unwrap();
        assert_eq!(result, 0xDEADBEEF);
    }

    #[test]
    fn test_octal() {
        let s = cstr_16!('0', '7', '7', '7');
        let result = atol(&s).unwrap();
        assert_eq!(result, 0o777);
    }

    #[test]
    fn test_leading_zero_decimal() {
        let s = cstr_16!('0', '1', '2', '3'); // Should be interpreted as octal
        let result = atol(&s).unwrap();
        assert_eq!(result, 0o123);
    }

    #[test]
    fn test_invalid_input() {
        let s = cstr_16!('x', 'y', 'z');
        let result = atol(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_string() {
        let s = cstr_16!();
        let result = atol(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_only_sign() {
        let s = cstr_16!('-');
        let result = atol(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_zero() {
        let s = cstr_16!('0');
        let result = atol(&s).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_negative_hex() {
        let s = cstr_16!('-', '0', 'x', '1', '0');
        let result = atol(&s).unwrap();
        assert_eq!(result, -16);
    }

    #[test]
    fn test_negative_octal() {
        let s = cstr_16!('-', '0', '7', '7');
        let result = atol(&s).unwrap();
        assert_eq!(result, -0o77);
    }

    ///
    fn cstr(s: &[u8]) -> *const char {
        s.as_ptr() as *const char
    }

    #[test]
    fn test_ascii_char() {
        let s = b"A";
        let ret = mbtowc(s).unwrap();
        assert_eq!(ret.0, 'A' as u16); // Unicode codepoint
        assert_eq!(ret.1, 1);
    }

    #[test]
    fn test_2byte_utf8() {
        let s = "é".as_bytes(); // UTF-8: 0xC3 0xA9
        let ret = mbtowc(s).unwrap();
        assert_eq!(ret.0, 0xE9); // Unicode codepoint
        assert_eq!(ret.1, 2);
    }

    #[test]
    fn test_3byte_utf8() {
        let s = "€".as_bytes(); // UTF-8: 0xE2 0x82 0xAC
        let ret = mbtowc(s).unwrap();
        assert_eq!(ret.0, 0x20AC); // Unicode codepoint for €
        assert_eq!(ret.1, 3);
    }

    // #[test]
    // fn test_4byte_utf8() {
    //     let s = "𐍈".as_bytes(); // UTF-8: 0xF0 0x90 0x8D 0x88
    //     let ret = mbtowc(s).unwrap();
    //     // Note: truncated to u16, high surrogate loss is expected
    //     assert_eq!(ret.0, 0xD800); // High surrogate part of 𐍈
    //     assert_eq!(ret.1, 4);
    // }

    #[test]
    fn test_invalid_utf8() {
        let s = &[0xFFu8]; // Invalid UTF-8
        let ret = mbtowc(s);
        assert!(ret.is_err());
    }

    #[test]
    fn test_empty_string_mbtowc() {
        let s: &[u8] = &[];
        let ret = mbtowc(s);
        assert!(ret.is_err());
    }

    #[test]
    fn test_insufficient_bytes() {
        let s = "€".as_bytes(); // 3-byte char
        // Pass only 2 bytes
        let ret = mbtowc(&s[..2]);
        assert!(ret.is_err());
    }
}
