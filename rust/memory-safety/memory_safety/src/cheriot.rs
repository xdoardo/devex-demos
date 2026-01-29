extern crate alloc;

unsafe extern "C" {
    pub fn cheriot_alloc(bytes: u32) -> *mut core::ffi::c_void;
    pub fn cheriot_free(ptr: *mut core::ffi::c_void);
    pub fn cheriot_panic();
    pub fn cheriot_print_str(v: *const core::ffi::c_char);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let str = alloc::string::ToString::to_string(&info.message());
    let str = <alloc::ffi::CString as core::str::FromStr>::from_str(&str).unwrap();
    unsafe {
        cheriot_print_str(str.as_ptr());
        drop(str);
        cheriot_panic()
    };
    loop {}
}

/// An allocator based on the CHERIoT RTOS allocator.
struct CHERIoTRTOSAllocator;

unsafe impl alloc::alloc::GlobalAlloc for CHERIoTRTOSAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { cheriot_alloc(layout.size() as _) as _ }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { cheriot_free(ptr as _) }
    }
}

#[global_allocator]
static CHERIOT_RTOS_ALLOCATOR: CHERIoTRTOSAllocator = CHERIoTRTOSAllocator;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::cheriot::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    let str = alloc::string::ToString::to_string(&args);
    let str = alloc::ffi::CString::new(str).unwrap();

    unsafe {
        cheriot_print_str(str.as_ptr());
    }

    drop(str);
}
