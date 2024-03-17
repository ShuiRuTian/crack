/// Surprisingly, [`std::ptr::copy`] faimly is slow when size is small
/// 
/// See:
/// - https://github.com/rust-lang/rust/issues/97022
/// - https://users.rust-lang.org/t/ptr-copy-nonoverlapping-slower-then-manual-per-byte-copy/75588
/// 
/// However, we nearly always use small cases
#[inline]
pub unsafe fn memcopy(src: *const u8, dst: *mut u8, count: usize){
    for i in 0..count{
        *dst.add(i) = *src.add(i);
    }
}