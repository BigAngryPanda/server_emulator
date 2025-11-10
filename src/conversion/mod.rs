pub unsafe fn strlen(data: *const u16) -> usize {
    let mut len = 0usize;

    while *data.add(len) != 0u16 {
        len += 1;
    }

    len
}

pub fn to_utf16_str(data: *const u16) -> &'static [u16] {
    unsafe { std::slice::from_raw_parts(data, strlen(data)) }
}

/// Convert utf16 string to *non-owning* vector
/// Do not forget [mem::forget](std::mem::forget)
pub fn to_utf16_vec(data: *const u16) -> Vec<u16> {
    unsafe { Vec::from_raw_parts(data as *mut u16, strlen(data), strlen(data)) }
}
