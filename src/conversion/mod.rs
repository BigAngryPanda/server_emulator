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

pub fn str_to_utf16(data: &String) -> Vec<u16> {
    let mut result: Vec<u16> = data.encode_utf16().collect();

    result.push(0);

    result
}

pub fn copy_unaligned<T: Copy>(src: *const T, dst: *mut T, count: usize) {
    for i in 0..count {
        unsafe {
            std::ptr::write_unaligned(dst.wrapping_add(i), src.wrapping_add(i).read());
        }
    }
}

pub trait FromInt64: Sized {
    fn from_i64(v: i64) -> Self;
}

impl FromInt64 for u32 {
    fn from_i64(v: i64) -> Self {
        v as u32
    }
}

impl FromInt64 for u64 {
    fn from_i64(v: i64) -> Self {
        v as u64
    }
}
