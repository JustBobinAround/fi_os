#![no_std]
#![no_main]

macro_rules! cstr_16 {
    [$($n:literal),*] => {
        [$($n as u16),*]
    };
}
use core::ffi::c_void;
use core::panic::PanicInfo;
use fi_uefi::{EFILoadedImageProtocol, EFISystemTable};

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    let st = EFISystemTable::fetch_global().unwrap();
    st.test_print(&cstr_16!('o', 'o', 'f'));
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn efi_main(image: *mut c_void, system_table: *mut c_void) -> usize {
    unsafe {
        EFILoadedImageProtocol::from_image_handle(image);
        EFISystemTable::set_system_table(system_table as *const EFISystemTable);
    }

    loop {}
    0
}
