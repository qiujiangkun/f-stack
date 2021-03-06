/* automatically generated by rust-bindgen */

pub type sa_family_t = ::std::os::raw::c_ushort;
pub type in_addr_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in_addr { pub s_addr: in_addr_t }

#[test]
fn bindgen_test_layout_in_addr() {
    assert_eq!(::std::mem::size_of::<in_addr>(), 4usize, concat!("Size of: ", stringify!(in_addr)));
    assert_eq!(::std::mem::align_of::<in_addr>(), 4usize, concat!("Alignment of ", stringify!(in_addr)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<in_addr>())).s_addr as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(in_addr), "::", stringify!(s_addr)));
}

pub type in_port_t = u16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_in { pub sin_family: sa_family_t, pub sin_port: in_port_t, pub sin_addr: in_addr, pub sin_zero: [::std::os::raw::c_uchar; 8usize] }

#[test]
fn bindgen_test_layout_sockaddr_in() {
    assert_eq!(::std::mem::size_of::<sockaddr_in>(), 16usize, concat!("Size of: ", stringify!(sockaddr_in)));
    assert_eq!(::std::mem::align_of::<sockaddr_in>(), 4usize, concat!("Alignment of ", stringify!(sockaddr_in)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_family as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(sockaddr_in), "::", stringify!(sin_family)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_port as *const _ as usize }, 2usize, concat!("Offset of field: ", stringify!(sockaddr_in), "::", stringify!(sin_port)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_addr as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(sockaddr_in), "::", stringify!(sin_addr)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_zero as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(sockaddr_in), "::", stringify!(sin_zero)));
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct buffer_t { pub size: ::std::os::raw::c_int, pub read: ::std::os::raw::c_int, pub write: ::std::os::raw::c_int, pub mask: ::std::os::raw::c_int, pub buf: *mut ::std::os::raw::c_char }

#[test]
fn bindgen_test_layout_buffer_t() {
    assert_eq!(::std::mem::size_of::<buffer_t>(), 24usize, concat!("Size of: ", stringify!(buffer_t)));
    assert_eq!(::std::mem::align_of::<buffer_t>(), 8usize, concat!("Alignment of ", stringify!(buffer_t)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<buffer_t>())).size as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(buffer_t), "::", stringify!(size)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<buffer_t>())).read as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(buffer_t), "::", stringify!(read)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<buffer_t>())).write as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(buffer_t), "::", stringify!(write)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<buffer_t>())).mask as *const _ as usize }, 12usize, concat!("Offset of field: ", stringify!(buffer_t), "::", stringify!(mask)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<buffer_t>())).buf as *const _ as usize }, 16usize, concat!("Offset of field: ", stringify!(buffer_t), "::", stringify!(buf)));
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dispatch_t { pub sockfd: ::std::os::raw::c_int, pub error_code: ::std::os::raw::c_int, pub read_buf: buffer_t, pub write_buf: buffer_t }

#[test]
fn bindgen_test_layout_dispatch_t() {
    assert_eq!(::std::mem::size_of::<dispatch_t>(), 56usize, concat!("Size of: ", stringify!(dispatch_t)));
    assert_eq!(::std::mem::align_of::<dispatch_t>(), 8usize, concat!("Alignment of ", stringify!(dispatch_t)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<dispatch_t>())).sockfd as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(dispatch_t), "::", stringify!(sockfd)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<dispatch_t>())).error_code as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(dispatch_t), "::", stringify!(error_code)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<dispatch_t>())).read_buf as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(dispatch_t), "::", stringify!(read_buf)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<dispatch_t>())).write_buf as *const _ as usize }, 32usize, concat!("Offset of field: ", stringify!(dispatch_t), "::", stringify!(write_buf)));
}

pub const commands_CONNECT: commands = 0;
pub const commands_CLOSE: commands = 1;
pub const commands_RECONNECT: commands = 2;

pub type commands = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct control_t { pub addr: sockaddr_in, pub fd: ::std::os::raw::c_int, pub command: commands, pub errno_number: ::std::os::raw::c_int, pub done: ::std::os::raw::c_int, pub dispatch: *mut dispatch_t }

#[test]
fn bindgen_test_layout_control_t() {
    assert_eq!(::std::mem::size_of::<control_t>(), 40usize, concat!("Size of: ", stringify!(control_t)));
    assert_eq!(::std::mem::align_of::<control_t>(), 8usize, concat!("Alignment of ", stringify!(control_t)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<control_t>())).addr as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(control_t), "::", stringify!(addr)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<control_t>())).fd as *const _ as usize }, 16usize, concat!("Offset of field: ", stringify!(control_t), "::", stringify!(fd)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<control_t>())).command as *const _ as usize }, 20usize, concat!("Offset of field: ", stringify!(control_t), "::", stringify!(command)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<control_t>())).errno_number as *const _ as usize }, 24usize, concat!("Offset of field: ", stringify!(control_t), "::", stringify!(errno_number)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<control_t>())).done as *const _ as usize }, 28usize, concat!("Offset of field: ", stringify!(control_t), "::", stringify!(done)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<control_t>())).dispatch as *const _ as usize }, 32usize, concat!("Offset of field: ", stringify!(control_t), "::", stringify!(dispatch)));
}

extern "C" { pub fn get_address(addr: *mut ::std::os::raw::c_char, port: ::std::os::raw::c_int) -> sockaddr_in; }

extern "C" { pub fn fstack_init(argc: ::std::os::raw::c_int, argv: *mut *mut ::std::os::raw::c_char); }

extern "C" { pub fn fstack_run(user_poll: ::std::option::Option<unsafe extern "C" fn()>); }