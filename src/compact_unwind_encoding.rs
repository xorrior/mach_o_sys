/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int_least8_t = int8_t;
pub type int_least16_t = int16_t;
pub type int_least32_t = int32_t;
pub type int_least64_t = int64_t;
pub type uint_least8_t = uint8_t;
pub type uint_least16_t = uint16_t;
pub type uint_least32_t = uint32_t;
pub type uint_least64_t = uint64_t;
pub type int_fast8_t = int8_t;
pub type int_fast16_t = int16_t;
pub type int_fast32_t = int32_t;
pub type int_fast64_t = int64_t;
pub type uint_fast8_t = uint8_t;
pub type uint_fast16_t = uint16_t;
pub type uint_fast32_t = uint32_t;
pub type uint_fast64_t = uint64_t;
pub type __int8_t = ::std::os::raw::c_char;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct __mbstate_t {
    pub _bindgen_data_: [u64; 16usize],
}
impl __mbstate_t {
    pub unsafe fn __mbstate8(&mut self)
     -> *mut [::std::os::raw::c_char; 128usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _mbstateL(&mut self) -> *mut ::std::os::raw::c_longlong {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for __mbstate_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for __mbstate_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                  *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
impl ::std::default::Default for __darwin_pthread_handler_rec {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
impl ::std::clone::Clone for _opaque_pthread_attr_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_attr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
impl ::std::clone::Clone for _opaque_pthread_cond_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_cond_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
impl ::std::default::Default for _opaque_pthread_condattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
impl ::std::clone::Clone for _opaque_pthread_mutex_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_mutex_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
impl ::std::default::Default for _opaque_pthread_mutexattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
impl ::std::default::Default for _opaque_pthread_once_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
impl ::std::clone::Clone for _opaque_pthread_rwlock_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_rwlock_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
impl ::std::default::Default for _opaque_pthread_rwlockattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
impl ::std::clone::Clone for _opaque_pthread_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
pub type compact_unwind_encoding_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed1 {
    UNWIND_IS_NOT_FUNCTION_START = 2147483648,
    UNWIND_HAS_LSDA = 1073741824,
    UNWIND_PERSONALITY_MASK = 805306368,
}
pub const UNWIND_X86_FRAMELESS_STACK_SIZE: Enum_Unnamed2 =
    Enum_Unnamed2::UNWIND_X86_EBP_FRAME_OFFSET;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed2 {
    UNWIND_X86_MODE_MASK = 251658240,
    UNWIND_X86_MODE_EBP_FRAME = 16777216,
    UNWIND_X86_MODE_STACK_IMMD = 33554432,
    UNWIND_X86_MODE_STACK_IND = 50331648,
    UNWIND_X86_MODE_DWARF = 67108864,
    UNWIND_X86_EBP_FRAME_REGISTERS = 32767,
    UNWIND_X86_EBP_FRAME_OFFSET = 16711680,
    UNWIND_X86_FRAMELESS_STACK_ADJUST = 57344,
    UNWIND_X86_FRAMELESS_STACK_REG_COUNT = 7168,
    UNWIND_X86_FRAMELESS_STACK_REG_PERMUTATION = 1023,
    UNWIND_X86_DWARF_SECTION_OFFSET = 16777215,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed3 {
    UNWIND_X86_REG_NONE = 0,
    UNWIND_X86_REG_EBX = 1,
    UNWIND_X86_REG_ECX = 2,
    UNWIND_X86_REG_EDX = 3,
    UNWIND_X86_REG_EDI = 4,
    UNWIND_X86_REG_ESI = 5,
    UNWIND_X86_REG_EBP = 6,
}
pub const UNWIND_X86_64_FRAMELESS_STACK_SIZE: Enum_Unnamed4 =
    Enum_Unnamed4::UNWIND_X86_64_RBP_FRAME_OFFSET;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed4 {
    UNWIND_X86_64_MODE_MASK = 251658240,
    UNWIND_X86_64_MODE_RBP_FRAME = 16777216,
    UNWIND_X86_64_MODE_STACK_IMMD = 33554432,
    UNWIND_X86_64_MODE_STACK_IND = 50331648,
    UNWIND_X86_64_MODE_DWARF = 67108864,
    UNWIND_X86_64_RBP_FRAME_REGISTERS = 32767,
    UNWIND_X86_64_RBP_FRAME_OFFSET = 16711680,
    UNWIND_X86_64_FRAMELESS_STACK_ADJUST = 57344,
    UNWIND_X86_64_FRAMELESS_STACK_REG_COUNT = 7168,
    UNWIND_X86_64_FRAMELESS_STACK_REG_PERMUTATION = 1023,
    UNWIND_X86_64_DWARF_SECTION_OFFSET = 16777215,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed5 {
    UNWIND_X86_64_REG_NONE = 0,
    UNWIND_X86_64_REG_RBX = 1,
    UNWIND_X86_64_REG_R12 = 2,
    UNWIND_X86_64_REG_R13 = 3,
    UNWIND_X86_64_REG_R14 = 4,
    UNWIND_X86_64_REG_R15 = 5,
    UNWIND_X86_64_REG_RBP = 6,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct unwind_info_section_header {
    pub version: uint32_t,
    pub commonEncodingsArraySectionOffset: uint32_t,
    pub commonEncodingsArrayCount: uint32_t,
    pub personalityArraySectionOffset: uint32_t,
    pub personalityArrayCount: uint32_t,
    pub indexSectionOffset: uint32_t,
    pub indexCount: uint32_t,
}
impl ::std::default::Default for unwind_info_section_header {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct unwind_info_section_header_index_entry {
    pub functionOffset: uint32_t,
    pub secondLevelPagesSectionOffset: uint32_t,
    pub lsdaIndexArraySectionOffset: uint32_t,
}
impl ::std::default::Default for unwind_info_section_header_index_entry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct unwind_info_section_header_lsda_index_entry {
    pub functionOffset: uint32_t,
    pub lsdaOffset: uint32_t,
}
impl ::std::default::Default for unwind_info_section_header_lsda_index_entry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct unwind_info_regular_second_level_entry {
    pub functionOffset: uint32_t,
    pub encoding: compact_unwind_encoding_t,
}
impl ::std::default::Default for unwind_info_regular_second_level_entry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct unwind_info_regular_second_level_page_header {
    pub kind: uint32_t,
    pub entryPageOffset: uint16_t,
    pub entryCount: uint16_t,
}
impl ::std::default::Default for unwind_info_regular_second_level_page_header
 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct unwind_info_compressed_second_level_page_header {
    pub kind: uint32_t,
    pub entryPageOffset: uint16_t,
    pub entryCount: uint16_t,
    pub encodingsPageOffset: uint16_t,
    pub encodingsCount: uint16_t,
}
impl ::std::default::Default for
 unwind_info_compressed_second_level_page_header {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
