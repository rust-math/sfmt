#![no_std]

/// 128-bit data structure
#[repr(C)]
pub union w128 {
    pub u: [u32; 4usize],
    pub u64: [u64; 2usize],
}

/// SFMT internal state
#[repr(C)]
pub struct SFMT {
    /// the 128-bit internal state array
    pub state: [w128; 156usize],
    /// index counter to the 32-bit internal state array
    pub idx: i32,
}

extern "C" {
    pub fn sfmt_fill_array32(sfmt: *mut SFMT, array: *mut u32, size: i32);
    pub fn sfmt_fill_array64(sfmt: *mut SFMT, array: *mut u64, size: i32);
    pub fn sfmt_gen_rand_all(sfmt: *mut SFMT);
    pub fn sfmt_get_idstring(sfmt: *mut SFMT) -> *const u8;
    pub fn sfmt_get_min_array_size32(sfmt: *mut SFMT) -> i32;
    pub fn sfmt_get_min_array_size64(sfmt: *mut SFMT) -> i32;
    pub fn sfmt_init_by_array(sfmt: *mut SFMT, init_key: *mut u32, key_length: i32);
    pub fn sfmt_init_gen_rand(sfmt: *mut SFMT, seed: u32);
}

#[cfg(test)]
mod test {
    #[test]
    fn bindgen_test_layout_w128() {
        assert_eq!(
            ::std::mem::size_of::<w128>(),
            16usize,
            concat!("Size of: ", stringify!(w128))
        );
        assert_eq!(
            ::std::mem::align_of::<w128>(),
            8usize,
            concat!("Alignment of ", stringify!(w128))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<w128>())).u as *const _ as usize },
            0usize,
            concat!("Offset of field: ", stringify!(w128), "::", stringify!(u))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<w128>())).u64 as *const _ as usize },
            0usize,
            concat!("Offset of field: ", stringify!(w128), "::", stringify!(u64))
        );
    }

    #[test]
    fn bindgen_test_layout_sfmt() {
        assert_eq!(
            ::std::mem::size_of::<SFMT>(),
            2504usize,
            concat!("Size of: ", stringify!(SFMT))
        );
        assert_eq!(
            ::std::mem::align_of::<SFMT>(),
            8usize,
            concat!("Alignment of ", stringify!(SFMT))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<SFMT>())).state as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(SFMT),
                "::",
                stringify!(state)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<SFMT>())).idx as *const _ as usize },
            2496usize,
            concat!("Offset of field: ", stringify!(SFMT), "::", stringify!(idx))
        );
    }
}
