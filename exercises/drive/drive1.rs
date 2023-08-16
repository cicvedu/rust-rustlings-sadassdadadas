// drive1.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.



fn modify_by_address(address: usize) {
    // `address` is a memory address, there is an u32 at that address. try modify
    // the u32's value to 0xAABBCCDD
    unsafe {
        let d: u32 = 0xAABBCCDD;
        core::ptr::copy_nonoverlapping(core::ptr::addr_of!(d) as *const u8, address as *const u8 as *mut u8, 4);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t:u32 = 0x12345678;
        modify_by_address(&mut t as *mut u32 as usize);
        assert!(t == 0xAABBCCDD);
    }
}
