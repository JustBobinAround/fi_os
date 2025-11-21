#![no_std]
use core::ffi::c_void;
pub type Wchar = u16;

#[repr(C)]
pub struct EFIDevicePath {
    ty: u8,
    sub_type: u8,
    len: [u8; 2],
}

#[repr(C)]
pub struct EFIOpenProtocolInfoEntry {
    agenct_handle: *mut c_void,
    controller_handle: *mut c_void,
    attributes: u32,
    open_count: u32,
}

#[derive(Clone)]
#[repr(C)]
pub struct GUID {
    data_1: u32,
    data_2: u16,
    data_3: u16,
    data_4: [u8; 8],
}

#[repr(C)]
pub struct EFIMemoryDescriptor {
    pub ty: u32,
    pub pad: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

#[repr(C)]
pub enum EFIAllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[derive(Clone)]
#[repr(C)]
pub enum EFIMemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType,
}

impl Default for EFIMemoryType {
    fn default() -> Self {
        EFIMemoryType::EfiLoaderData
    }
}

#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc_32: u32,
    pub reserved: u32,
}

// unsafe extern "efiapi" {
//     pub static mut LIP: *mut EFILoadedImageProtocol;
//     pub static mut BS: *mut EFIBootServices;
//     // pub static mut ST: *mut EFISystemTable;
// }

#[repr(C)]
pub struct EFILoadedImageProtocol {
    pub revision: u32,
    pub parent_handle: u64,
    pub system_table: *mut c_void,
    pub device_handle: u64,
    pub file_path: *mut EFIDevicePath,
    pub reserved: *mut c_void,
    pub load_options_size: u32,
    pub load_options: *mut c_void,
    pub image_base: *mut c_void,
    pub image_size: u64,
    pub image_code_type: u32,
    pub image_data_type: u32,
}
use core::sync::atomic::{AtomicPtr, Ordering};
static LIP: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());
static ST: AtomicPtr<EFISystemTable> = AtomicPtr::new(core::ptr::null_mut());
static BS: AtomicPtr<EFIBootServices> = AtomicPtr::new(core::ptr::null_mut());

impl EFILoadedImageProtocol {
    pub unsafe fn from_image_handle(image_handle: *mut c_void) {
        LIP.store(image_handle, Ordering::Release);
    }

    pub fn fetch_global() -> Option<&'static Self> {
        let ptr = LIP.load(Ordering::Acquire);
        if ptr.is_null() {
            None
        } else {
            let p_self = ptr as *mut Self;
            Some(unsafe { &*p_self })
        }
    }
    // pub fn from_image_handle(handle: *mut c_void) -> Option<&'static Self> {
    //     unsafe { LIP.as_ref() }
    // }

    pub fn global_image_data_type() -> u32 {
        match Self::fetch_global() {
            Some(lip) => lip.image_data_type,
            None => 2,
        }
    }
}
#[repr(C)]
pub struct EFIInputKey {
    scan_code: u16,
    unicode_char: Wchar,
}

#[repr(C)]
pub struct SimpleInputInterface {
    reset: unsafe extern "efiapi" fn(this: *mut c_void, u8) -> u64,
    read_key_stroke: unsafe extern "efiapi" fn(this: *mut c_void, key: *mut EFIInputKey) -> u64,
    wait_for_key: *mut c_void,
}

#[repr(C)]
pub struct SimpleTextOutputMode {
    max_mode: i32,
    mode: i32,
    attribute: i32,
    cursor_column: i32,
    cursor_row: i32,
    cursor_visible: u8,
}

#[repr(C)]
pub struct SimpleTextOutputInterface {
    pub reset: unsafe extern "efiapi" fn(this: *mut Self, extended_verification: u8) -> u64,
    pub output_string: unsafe extern "efiapi" fn(this: *mut Self, string: *mut Wchar) -> u64,
    pub test_string: unsafe extern "efiapi" fn(this: *mut Self, string: *mut Wchar) -> u64,
    pub query_mode: unsafe extern "efiapi" fn(this: *mut Self, mode_number: u64) -> u64,
    pub set_mode: unsafe extern "efiapi" fn(this: *mut Self, mode_number: u64) -> u64,
    pub set_attribute: unsafe extern "efiapi" fn(this: *mut Self, attribute: u64) -> u64,
    pub clear_screen: unsafe extern "efiapi" fn(this: *mut Self) -> u64,
    pub set_cursor_position:
        unsafe extern "efiapi" fn(this: *mut Self, column: u64, row: u64) -> u64,
    pub enable_cursor: unsafe extern "efiapi" fn(this: *mut Self, enable: u8) -> u64,
    pub mode: *mut SimpleTextOutputMode,
}

impl SimpleTextOutputInterface {
    pub fn output_string(&mut self, s: &[u16]) {
        //TODO err handling
        unsafe { (self.output_string)(self, s.as_ptr() as *mut Wchar) };
    }
}

#[repr(C)]
pub struct EFITimeCapabilities {
    pub resolution: u32,
    pub accuracy: u32,
    pub sets_to_zero: u8,
}

#[repr(C)]
pub struct EFITime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub pad_1: u8,
    pub nanosecond: u32,
    pub timezone: i16,
    pub daylight: u8,
    pub pad_2: u8,
}

#[repr(C)]
pub enum EFIResetType {
    EfiResetCold,
    EfiResetWarm,
    EfiResetShutdown,
}

#[repr(C)]
pub struct EFICapsuleHeader {
    pub capsule_guid: GUID,
    pub header_size: u32,
    pub flags: u32,
    pub capsule_image_size: u32,
}

#[repr(C)]
pub struct EFIRuntimeServices {
    pub header: TableHeader,
    pub get_time: unsafe extern "efiapi" fn(
        time: *mut EFITime,
        capabilities: *mut EFITimeCapabilities,
    ) -> u64,
    pub set_time: unsafe extern "efiapi" fn(time: *mut EFITime) -> u64,
    pub get_wakeup_time:
        unsafe extern "efiapi" fn(enable: *mut u8, pending: *mut u8, time: *mut EFITime) -> u64,
    pub set_wakeup_time: unsafe extern "efiapi" fn(enable: u8, time: *mut EFITime) -> u64,
    pub set_virtual_address_map: unsafe extern "efiapi" fn(
        memory_map_size: u64,
        descriptor_size: u64,
        descriptor_version: u32,
        virtual_map: *mut EFIMemoryDescriptor,
    ) -> u64,
    pub convert_pointer:
        unsafe extern "efiapi" fn(debug_disposition: u64, address: *mut *mut c_void) -> u64,
    pub get_variable: unsafe extern "efiapi" fn(
        variable_name: Wchar,
        vendor_guid: *mut GUID,
        attributes: *mut u32,
        data_size: *mut u64,
        data: *mut c_void,
    ) -> u64,
    pub get_next_variable: unsafe extern "efiapi" fn(
        variable_name_size: *mut u64,
        variable_name: *mut Wchar,
        vendor_guid: *mut GUID,
    ) -> u64,
    pub set_variable: unsafe extern "efiapi" fn(
        variable_name: *mut Wchar,
        vendor_guid: *mut GUID,
        attributes: u32,
        data_size: u64,
        data: *mut c_void,
    ) -> u64,
    pub get_next_high_mono: unsafe extern "efiapi" fn(count: *mut u64) -> u64,
    pub reset_system_type: unsafe extern "efiapi" fn(
        reset_type: EFIResetType,
        reset_status: u64,
        data_size: u64,
        reset_data: *mut Wchar,
    ) -> u64,
    pub update_capsule: unsafe extern "efiapi" fn(
        capsule_header_array: *mut *mut EFICapsuleHeader,
        capsule_count: u64,
        scatter_gather_list: u64,
    ) -> u64,
    pub query_capsule_capabilities: unsafe extern "efiapi" fn(
        capsule_header_array: *mut *mut EFICapsuleHeader,
        capsule_count: u64,
        maximum_capsule_size: u64,
        reset_type: *mut EFIResetType,
    ) -> u64,
    pub query_variable_info: unsafe extern "efiapi" fn(
        attributes: u32,
        maximum_variable_storage_size: *mut u64,
        remaining_variable_storage_size: *mut u64,
        maximum_variable_size: *mut u64,
    ) -> u64,
}

#[repr(C)]
pub struct EFIConfigurationTable {
    pub vendor_guid: GUID,
    pub vendor_table: *mut c_void,
}

#[repr(C)]
pub struct EFISystemTable {
    pub header: TableHeader,
    pub firmware_vendor: *mut u16,
    pub firmware_revision: u32,
    pub stdin_handle: *mut c_void,
    pub stdin: *mut SimpleInputInterface,
    pub stdout_handle: *mut c_void,
    pub stdout: *mut SimpleTextOutputInterface,
    pub stderr_handle: *mut c_void,
    pub stderr: *mut SimpleTextOutputInterface,
    pub runtime_services: *mut EFIRuntimeServices,
    pub boot_services: *mut EFIBootServices,
    pub number_of_table_entries: u64,
    pub configuration_table: *mut EFIConfigurationTable,
}

impl EFISystemTable {
    pub unsafe fn set_system_table(ptr: *const EFISystemTable) {
        ST.store(ptr.cast_mut(), Ordering::Release);
        let st = Self::fetch_global().expect("Failed to set ST");
        let p_bt = st.boot_services;
        unsafe {
            BS.store(
                p_bt.as_mut().expect("Failed to get boot services ref"),
                Ordering::Release,
            )
        };
    }

    pub fn fetch_global() -> Option<&'static Self> {
        let ptr = ST.load(Ordering::Acquire);
        if ptr.is_null() {
            None
        } else {
            let p_self = ptr as *mut Self;
            Some(unsafe { &*p_self })
        }
    }

    pub fn test_print(&self, s: &[u16]) {
        let stdout = unsafe { self.stdout.as_mut().unwrap() };
        stdout.output_string(s);
    }
}
#[repr(C)]
pub struct EFIBootServices {
    pub hdr: TableHeader,

    // TODO: need to do research
    // not fully sure what these should actually be...
    // pub reset: unsafe extern "efiapi" fn(this: *mut Self, extended: u8) -> usize,
    pub raise_tpl: unsafe extern "efiapi" fn(new_tpl: usize) -> usize,
    pub restor_tpl: unsafe extern "efiapi" fn(old_tpl: usize) -> usize,

    pub allocate_pages: unsafe extern "efiapi" fn(
        alloc_ty: EFIAllocateType,
        mem_ty: EFIMemoryType,
        count: usize,
        addr: *mut u64,
    ) -> u64,
    pub free_pages: unsafe extern "efiapi" fn(addr: u64, pages: usize) -> u64,
    pub get_memory_map: unsafe extern "efiapi" fn(
        size: *mut usize,
        map: *mut EFIMemoryDescriptor,
        key: *mut usize,
        desc_size: *mut usize,
        desc_version: *mut u32,
    ) -> u64,
    pub allocate_pool:
        unsafe extern "efiapi" fn(pool_type: u32, size: usize, buffer: *mut *mut c_void) -> u64,
    pub free_pool: unsafe extern "efiapi" fn(buffer: *mut c_void) -> u64,

    pub create_event: unsafe extern "efiapi" fn(
        ty: u32,
        notify_tpl: u64,
        notify_func: Option<unsafe extern "efiapi" fn(event: *mut c_void, context: *mut c_void)>,
        notify_ctx: *mut c_void,
        out_event: *mut c_void,
    ) -> u64,
    pub set_timer: unsafe extern "efiapi" fn(event: c_void, ty: i32, trigger_time: u64) -> u64,
    pub wait_for_event: unsafe extern "efiapi" fn(
        number_of_events: u64,
        events: *mut c_void,
        out_index: *mut u64,
    ) -> u64,
    pub signal_event: unsafe extern "efiapi" fn(event: c_void) -> u64,
    pub close_event: unsafe extern "efiapi" fn(event: c_void) -> u64,
    pub check_event: unsafe extern "efiapi" fn(event: c_void) -> u64,

    // Protocol handlers
    pub install_protocol_interface: unsafe extern "efiapi" fn(
        handle: *mut c_void,
        guid: *const GUID,
        interface_type: u32,
        interface: *const c_void,
    ) -> u64,
    pub reinstall_protocol_interface: unsafe extern "efiapi" fn(
        handle: c_void,
        protocol: *const GUID,
        old_interface: *const c_void,
        new_interface: *const c_void,
    ) -> u64,
    pub uninstall_protocol_interface: unsafe extern "efiapi" fn(
        handle: c_void,
        protocol: *const GUID,
        interface: *const c_void,
    ) -> u64,
    pub handle_protocol: unsafe extern "efiapi" fn(
        handle: c_void,
        protocol: *const GUID,
        out_proto: *mut *mut c_void,
    ) -> u64,
    pub reserved: *mut c_void,
    pub register_protocol_notify: unsafe extern "efiapi" fn(
        protocol: *const GUID,
        event: c_void,
        registration: *mut *const c_void,
    ) -> u64,
    pub locate_handle: unsafe extern "efiapi" fn(
        search_ty: i32,
        protocol: *const GUID,
        key: *const c_void,
        buf_sz: *mut u64,
        buf: *mut c_void,
    ) -> u64,
    pub locate_device_path: unsafe extern "efiapi" fn(
        protocol: *const GUID,
        device: *mut *const EFIDevicePath,
        out_handle: *mut c_void,
    ) -> u64,
    pub install_configuration_table:
        unsafe extern "efiapi" fn(guid_entry: *const GUID, table_ptr: *const c_void) -> u64,

    // Image services
    pub load_image: unsafe extern "efiapi" fn(
        boot_policy: u8,
        parent_image_handle: *mut c_void,
        device: *const EFIDevicePath,
        source_buffer: *const u8,
        source_size: u64,
        image_handle: *mut *mut c_void,
    ) -> u64,
    pub start_image: unsafe extern "efiapi" fn(
        image_handle: *mut c_void,
        exit_data_size: *mut u64,
        exit_data: *mut *mut u16,
    ) -> u64,
    pub exit: unsafe extern "efiapi" fn(
        image_handle: c_void,
        exit_status: u64,
        exit_data_size: u64,
        exit_data: *mut u16,
    ) -> !,
    pub unload_image: unsafe extern "efiapi" fn(image_handle: c_void) -> u64,
    pub exit_boot_services: unsafe extern "efiapi" fn(image_handle: c_void, map_key: u64) -> u64,

    // Misc services
    pub get_next_monotonic_count: unsafe extern "efiapi" fn(count: *mut u64) -> u64,
    pub stall: unsafe extern "efiapi" fn(microseconds: u64) -> u64,
    pub set_watchdog_timer: unsafe extern "efiapi" fn(
        timeout: u64,
        watchdog_code: u64,
        data_size: u64,
        watchdog_data: *const u16,
    ) -> u64,

    // Driver support services
    pub connect_controller: unsafe extern "efiapi" fn(
        controller: c_void,
        driver_image: c_void,
        remaining_device_path: *const EFIDevicePath,
        recursive: u8,
    ) -> u64,
    pub disconnect_controller:
        unsafe extern "efiapi" fn(controller: c_void, driver_image: c_void, child: c_void) -> u64,

    // Protocol open / close services
    pub open_protocol: unsafe extern "efiapi" fn(
        handle: c_void,
        protocol: *const GUID,
        interface: *mut *mut c_void,
        agent_handle: c_void,
        controller_handle: c_void,
        attributes: u32,
    ) -> u64,
    pub close_protocol: unsafe extern "efiapi" fn(
        handle: c_void,
        protocol: *const GUID,
        agent_handle: c_void,
        controller_handle: c_void,
    ) -> u64,
    pub open_protocol_information: unsafe extern "efiapi" fn(
        handle: c_void,
        protocol: *const GUID,
        entry_buffer: *mut *const EFIOpenProtocolInfoEntry,
        entry_count: *mut u64,
    ) -> u64,
    pub protocols_per_handle: unsafe extern "efiapi" fn(
        handle: c_void,
        protocol_buffer: *mut *mut *const GUID,
        protocol_buffer_count: *mut u64,
    ) -> u64,
    pub locate_handle_buffer: unsafe extern "efiapi" fn(
        search_ty: i32,
        protocol: *const GUID,
        key: *const c_void,
        no_handles: *mut u64,
        buf: *mut *mut c_void,
    ) -> u64,
    pub locate_protocol: unsafe extern "efiapi" fn(
        protocol: *const GUID,
        registration: *mut c_void,
        out_proto: *mut *mut c_void,
    ) -> u64,

    pub install_multiple_protocol_interfaces: unsafe extern "C" fn(handle: *mut c_void, ...) -> u64,
    pub uninstall_multiple_protocol_interfaces: unsafe extern "C" fn(handle: c_void, ...) -> u64,
    pub calculate_crc32:
        unsafe extern "efiapi" fn(data: *const c_void, data_size: u64, crc32: *mut u32) -> u64,
    pub copy_mem: unsafe extern "efiapi" fn(dest: *mut u8, src: *const u8, len: u64),
    pub set_mem: unsafe extern "efiapi" fn(buffer: *mut u8, len: u64, value: u8),

    // New event functions (UEFI 2.0 or newer)
    pub create_event_ex: unsafe extern "efiapi" fn(
        ty: u32,
        notify_tpl: u64,
        notify_fn: Option<unsafe extern "efiapi" fn(event: *mut c_void, context: *mut c_void)>,
        notify_ctx: *mut c_void,
        event_group: *mut GUID,
        out_event: *mut c_void,
    ) -> u64,
}

impl EFIBootServices {
    pub fn fetch_global() -> Option<&'static Self> {
        let ptr = BS.load(Ordering::Acquire);
        if ptr.is_null() {
            None
        } else {
            let p_self = ptr as *mut Self;
            Some(unsafe { &*p_self })
        }
    }

    pub fn allocate_pool(&self, pool_type: u32, size: usize) -> *mut u8 {
        macro_rules! cstr_16 {
            [$($n:literal),*] => {
                [$($n as u16),*]
            };
        }
        let st = EFISystemTable::fetch_global().unwrap();
        let mut buffer = core::ptr::null_mut();
        unsafe { (self.allocate_pool)(2, size, &mut buffer) };

        buffer as *mut u8
    }
    pub fn free_pool(&self, buffer: *mut u8) {
        unsafe { (self.free_pool)(buffer as *mut c_void) };
    }

    // : unsafe extern "efiapi" fn(
    //     pool_type: EFIMemoryType,
    //     size: u64,
    //     buffer: *mut *mut c_void,
    // ) -> u64,
}
