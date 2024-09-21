pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl<T> AsBytes for Vec<T> {
    fn as_bytes(&self) -> &[u8] {
        // Get a pointer to the data and calculate the length in bytes
        let ptr = self.as_ptr() as *const u8;
        let len = self.len() * std::mem::size_of::<T>();

        // Convert the pointer and length to a byte slice
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

#[test]
fn test_as_bytes() {
    let a = vec![1i32, 1000i32, 0i32, 1i32];
    let b = a.as_bytes();
    dbg!(b);
}