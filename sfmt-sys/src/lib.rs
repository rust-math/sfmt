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

/// the 128-bit internal state array
#[repr(C)]
pub struct DSFMT {
    pub status: [w128; 192usize],
    pub idx: i32,
}

extern "C" {
    #[link_name = "\u{1}dsfmt_global_data"]
    pub static mut dsfmt_global_data: DSFMT;
    #[link_name = "\u{1}dsfmt_global_mexp"]
    pub static mut dsfmt_global_mexp: i32;
    pub fn dsfmt_gen_rand_all(dsfmt: *mut DSFMT);
    pub fn dsfmt_fill_array_open_close(dsfmt: *mut DSFMT, array: *mut f64, size: i32);
    pub fn dsfmt_fill_array_close_open(dsfmt: *mut DSFMT, array: *mut f64, size: i32);
    pub fn dsfmt_fill_array_open_open(dsfmt: *mut DSFMT, array: *mut f64, size: i32);
    pub fn dsfmt_fill_array_close1_open2(dsfmt: *mut DSFMT, array: *mut f64, size: i32);
    pub fn dsfmt_chk_init_gen_rand(dsfmt: *mut DSFMT, seed: u32, mexp: i32);
    pub fn dsfmt_chk_init_by_array(
        dsfmt: *mut DSFMT,
        init_key: *mut u32,
        key_length: i32,
        mexp: i32,
    );
    pub fn dsfmt_get_idstring() -> *const i8;
    pub fn dsfmt_get_min_array_size() -> i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bindgen_test_layout_w128() {
        assert_eq!(
            ::core::mem::size_of::<w128>(),
            16usize,
            concat!("Size of: ", stringify!(w128))
        );
        assert_eq!(
            ::core::mem::align_of::<w128>(),
            8usize,
            concat!("Alignment of ", stringify!(w128))
        );
        assert_eq!(
            unsafe { &(*(::core::ptr::null::<w128>())).u as *const _ as usize },
            0usize,
            concat!("Offset of field: ", stringify!(w128), "::", stringify!(u))
        );
        assert_eq!(
            unsafe { &(*(::core::ptr::null::<w128>())).u64 as *const _ as usize },
            0usize,
            concat!("Offset of field: ", stringify!(w128), "::", stringify!(u64))
        );
    }

    #[test]
    fn bindgen_test_layout_sfmt() {
        assert_eq!(
            ::core::mem::size_of::<SFMT>(),
            2504usize,
            concat!("Size of: ", stringify!(SFMT))
        );
        assert_eq!(
            ::core::mem::align_of::<SFMT>(),
            8usize,
            concat!("Alignment of ", stringify!(SFMT))
        );
        assert_eq!(
            unsafe { &(*(::core::ptr::null::<SFMT>())).state as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(SFMT),
                "::",
                stringify!(state)
            )
        );
        assert_eq!(
            unsafe { &(*(::core::ptr::null::<SFMT>())).idx as *const _ as usize },
            2496usize,
            concat!("Offset of field: ", stringify!(SFMT), "::", stringify!(idx))
        );
    }

    #[test]
    fn bindgen_test_layout_dsfmt() {
        assert_eq!(
            ::core::mem::size_of::<DSFMT>(),
            3080usize,
            concat!("Size of: ", stringify!(DSFMT))
        );
        assert_eq!(
            ::core::mem::align_of::<DSFMT>(),
            8usize,
            concat!("Alignment of ", stringify!(DSFMT))
        );
        assert_eq!(
            unsafe { &(*(::core::ptr::null::<DSFMT>())).status as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(DSFMT),
                "::",
                stringify!(status)
            )
        );
        assert_eq!(
            unsafe { &(*(::core::ptr::null::<DSFMT>())).idx as *const _ as usize },
            3072usize,
            concat!(
                "Offset of field: ",
                stringify!(DSFMT),
                "::",
                stringify!(idx)
            )
        );
    }
}
