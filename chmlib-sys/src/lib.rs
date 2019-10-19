/* automatically generated by rust-bindgen */

pub const CHM_UNCOMPRESSED: u32 = 0;
pub const CHM_COMPRESSED: u32 = 1;
pub const CHM_MAX_PATHLEN: u32 = 512;
pub const CHM_PARAM_MAX_BLOCKS_CACHED: u32 = 0;
pub const CHM_RESOLVE_SUCCESS: u32 = 0;
pub const CHM_RESOLVE_FAILURE: u32 = 1;
pub const CHM_ENUMERATE_NORMAL: u32 = 1;
pub const CHM_ENUMERATE_META: u32 = 2;
pub const CHM_ENUMERATE_SPECIAL: u32 = 4;
pub const CHM_ENUMERATE_FILES: u32 = 8;
pub const CHM_ENUMERATE_DIRS: u32 = 16;
pub const CHM_ENUMERATE_ALL: u32 = 31;
pub const CHM_ENUMERATOR_FAILURE: u32 = 0;
pub const CHM_ENUMERATOR_CONTINUE: u32 = 1;
pub const CHM_ENUMERATOR_SUCCESS: u32 = 2;
pub type LONGUINT64 = ::std::os::raw::c_ulonglong;
pub type LONGINT64 = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct chmFile {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chmUnitInfo {
    pub start: LONGUINT64,
    pub length: LONGUINT64,
    pub space: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub path: [::std::os::raw::c_char; 513usize],
}
#[test]
fn bindgen_test_layout_chmUnitInfo() {
    assert_eq!(
        ::std::mem::size_of::<chmUnitInfo>(),
        544usize,
        concat!("Size of: ", stringify!(chmUnitInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<chmUnitInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(chmUnitInfo))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<chmUnitInfo>())).start as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(chmUnitInfo),
            "::",
            stringify!(start)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<chmUnitInfo>())).length as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(chmUnitInfo),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<chmUnitInfo>())).space as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(chmUnitInfo),
            "::",
            stringify!(space)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<chmUnitInfo>())).flags as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(chmUnitInfo),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<chmUnitInfo>())).path as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(chmUnitInfo),
            "::",
            stringify!(path)
        )
    );
}
extern "C" {
    pub fn chm_open(filename: *const ::std::os::raw::c_char) -> *mut chmFile;
}
extern "C" {
    pub fn chm_close(h: *mut chmFile);
}
extern "C" {
    pub fn chm_set_param(
        h: *mut chmFile,
        paramType: ::std::os::raw::c_int,
        paramVal: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn chm_resolve_object(
        h: *mut chmFile,
        objPath: *const ::std::os::raw::c_char,
        ui: *mut chmUnitInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chm_retrieve_object(
        h: *mut chmFile,
        ui: *mut chmUnitInfo,
        buf: *mut ::std::os::raw::c_uchar,
        addr: LONGUINT64,
        len: LONGINT64,
    ) -> LONGINT64;
}
pub type CHM_ENUMERATOR = ::std::option::Option<
    unsafe extern "C" fn(
        h: *mut chmFile,
        ui: *mut chmUnitInfo,
        context: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn chm_enumerate(
        h: *mut chmFile,
        what: ::std::os::raw::c_int,
        e: CHM_ENUMERATOR,
        context: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chm_enumerate_dir(
        h: *mut chmFile,
        prefix: *const ::std::os::raw::c_char,
        what: ::std::os::raw::c_int,
        e: CHM_ENUMERATOR,
        context: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
