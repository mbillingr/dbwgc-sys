/* automatically generated by rust-bindgen */

pub const GC_TMP_VERSION_MAJOR: u32 = 8;
pub const GC_TMP_VERSION_MINOR: u32 = 1;
pub const GC_TMP_VERSION_MICRO: u32 = 0;
pub const GC_VERSION_MAJOR: u32 = 8;
pub const GC_VERSION_MINOR: u32 = 1;
pub const GC_VERSION_MICRO: u32 = 0;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _ARGMAX: u32 = 100;
pub const _CRT_INT_MAX: u32 = 2147483647;
pub const _CRT_FUNCTIONS_REQUIRED: u32 = 1;
pub const _CRT_HAS_CXX17: u32 = 0;
pub const _ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE: u32 = 1;
pub const _CRT_BUILD_DESKTOP_APP: u32 = 1;
pub const _CRT_INTERNAL_NONSTDC_NAMES: u32 = 1;
pub const __STDC_SECURE_LIB__: u32 = 200411;
pub const __GOT_SECURE_LIB__: u32 = 200411;
pub const __STDC_WANT_SECURE_LIB__: u32 = 1;
pub const _SECURECRT_FILL_BUFFER_PATTERN: u32 = 254;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_COUNT: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES: u32 = 1;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_MEMORY: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES_MEMORY: u32 = 0;
pub const GC_TIME_UNLIMITED: u32 = 999999;
pub const GC_PROTECTS_POINTER_HEAP: u32 = 1;
pub const GC_PROTECTS_PTRFREE_HEAP: u32 = 2;
pub const GC_PROTECTS_STATIC_DATA: u32 = 4;
pub const GC_PROTECTS_STACK: u32 = 8;
pub const GC_PROTECTS_NONE: u32 = 0;
pub const GC_NO_MEMORY: u32 = 2;
pub const GC_SUCCESS: u32 = 0;
pub const GC_DUPLICATE: u32 = 1;
pub const GC_NO_THREADS: u32 = 2;
pub const GC_UNIMPLEMENTED: u32 = 3;
pub const GC_NOT_FOUND: u32 = 4;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type __crt_bool = bool;
extern "C" {
    pub fn _invalid_parameter_noinfo();
}
extern "C" {
    pub fn _invalid_parameter_noinfo_noreturn();
}
extern "C" {
    pub fn _invoke_watson(
        _Expression: *const wchar_t,
        _FunctionName: *const wchar_t,
        _FileName: *const wchar_t,
        _LineNo: ::std::os::raw::c_uint,
        _Reserved: usize,
    );
}
pub type errno_t = ::std::os::raw::c_int;
pub type wint_t = ::std::os::raw::c_ushort;
pub type wctype_t = ::std::os::raw::c_ushort;
pub type __time32_t = ::std::os::raw::c_long;
pub type __time64_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data_public {
    pub _locale_pctype: *const ::std::os::raw::c_ushort,
    pub _locale_mb_cur_max: ::std::os::raw::c_int,
    pub _locale_lc_codepage: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___crt_locale_data_public() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_data_public>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_data_public>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_pctype as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_pctype)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_mb_cur_max as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_mb_cur_max)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_lc_codepage as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_lc_codepage)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_data,
    pub mbcinfo: *mut __crt_multibyte_data,
}
#[test]
fn bindgen_test_layout___crt_locale_pointers() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_pointers>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_pointers>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__crt_locale_pointers>())).locinfo as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(locinfo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__crt_locale_pointers>())).mbcinfo as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(mbcinfo)
        )
    );
}
pub type _locale_t = *mut __crt_locale_pointers;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout__Mbstatet() {
    assert_eq!(
        ::std::mem::size_of::<_Mbstatet>(),
        8usize,
        concat!("Size of: ", stringify!(_Mbstatet))
    );
    assert_eq!(
        ::std::mem::align_of::<_Mbstatet>(),
        4usize,
        concat!("Alignment of ", stringify!(_Mbstatet))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._Wchar as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Wchar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._Byte as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Byte)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._State as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_State)
        )
    );
}
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = usize;
extern "C" {
    pub fn _errno() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn _set_errno(_Value: ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    pub fn _get_errno(_Value: *mut ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    pub fn __threadid() -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn __threadhandle() -> usize;
}
pub type GC_PTR = *mut ::std::os::raw::c_void;
pub type GC_word = ::std::os::raw::c_ulonglong;
pub type GC_signed_word = ::std::os::raw::c_longlong;
extern "C" {
    pub fn GC_get_version() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub static mut GC_gc_no: GC_word;
}
extern "C" {
    pub fn GC_get_gc_no() -> GC_word;
}
pub type GC_oom_func =
    ::std::option::Option<unsafe extern "C" fn(arg1: usize) -> *mut ::std::os::raw::c_void>;
extern "C" {
    pub static mut GC_oom_fn: GC_oom_func;
}
extern "C" {
    pub fn GC_set_oom_fn(arg1: GC_oom_func);
}
extern "C" {
    pub fn GC_get_oom_fn() -> GC_oom_func;
}
pub type GC_on_heap_resize_proc = ::std::option::Option<unsafe extern "C" fn(arg1: GC_word)>;
extern "C" {
    pub static mut GC_on_heap_resize: GC_on_heap_resize_proc;
}
extern "C" {
    pub fn GC_set_on_heap_resize(arg1: GC_on_heap_resize_proc);
}
extern "C" {
    pub fn GC_get_on_heap_resize() -> GC_on_heap_resize_proc;
}
pub const GC_EventType_GC_EVENT_START: GC_EventType = 0;
pub const GC_EventType_GC_EVENT_MARK_START: GC_EventType = 1;
pub const GC_EventType_GC_EVENT_MARK_END: GC_EventType = 2;
pub const GC_EventType_GC_EVENT_RECLAIM_START: GC_EventType = 3;
pub const GC_EventType_GC_EVENT_RECLAIM_END: GC_EventType = 4;
pub const GC_EventType_GC_EVENT_END: GC_EventType = 5;
pub const GC_EventType_GC_EVENT_PRE_STOP_WORLD: GC_EventType = 6;
pub const GC_EventType_GC_EVENT_POST_STOP_WORLD: GC_EventType = 7;
pub const GC_EventType_GC_EVENT_PRE_START_WORLD: GC_EventType = 8;
pub const GC_EventType_GC_EVENT_POST_START_WORLD: GC_EventType = 9;
pub const GC_EventType_GC_EVENT_THREAD_SUSPENDED: GC_EventType = 10;
pub const GC_EventType_GC_EVENT_THREAD_UNSUSPENDED: GC_EventType = 11;
pub type GC_EventType = i32;
pub type GC_on_collection_event_proc =
    ::std::option::Option<unsafe extern "C" fn(arg1: GC_EventType)>;
extern "C" {
    pub fn GC_set_on_collection_event(arg1: GC_on_collection_event_proc);
}
extern "C" {
    pub fn GC_get_on_collection_event() -> GC_on_collection_event_proc;
}
extern "C" {
    pub static mut GC_find_leak: ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_find_leak(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_find_leak() -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut GC_all_interior_pointers: ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_all_interior_pointers(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_all_interior_pointers() -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut GC_finalize_on_demand: ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_finalize_on_demand(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_finalize_on_demand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut GC_java_finalization: ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_java_finalization(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_java_finalization() -> ::std::os::raw::c_int;
}
pub type GC_finalizer_notifier_proc = ::std::option::Option<unsafe extern "C" fn()>;
extern "C" {
    pub static mut GC_finalizer_notifier: GC_finalizer_notifier_proc;
}
extern "C" {
    pub fn GC_set_finalizer_notifier(arg1: GC_finalizer_notifier_proc);
}
extern "C" {
    pub fn GC_get_finalizer_notifier() -> GC_finalizer_notifier_proc;
}
extern "C" {
    pub static mut GC_dont_gc: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut GC_dont_expand: ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_dont_expand(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_dont_expand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut GC_use_entire_heap: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut GC_full_freq: ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_full_freq(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_full_freq() -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut GC_non_gc_bytes: GC_word;
}
extern "C" {
    pub fn GC_set_non_gc_bytes(arg1: GC_word);
}
extern "C" {
    pub fn GC_get_non_gc_bytes() -> GC_word;
}
extern "C" {
    pub static mut GC_no_dls: ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_no_dls(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_no_dls() -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut GC_free_space_divisor: GC_word;
}
extern "C" {
    pub fn GC_set_free_space_divisor(arg1: GC_word);
}
extern "C" {
    pub fn GC_get_free_space_divisor() -> GC_word;
}
extern "C" {
    pub static mut GC_max_retries: GC_word;
}
extern "C" {
    pub fn GC_set_max_retries(arg1: GC_word);
}
extern "C" {
    pub fn GC_get_max_retries() -> GC_word;
}
extern "C" {
    pub static mut GC_stackbottom: *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut GC_dont_precollect: ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_dont_precollect(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_dont_precollect() -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut GC_time_limit: ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn GC_set_time_limit(arg1: ::std::os::raw::c_ulong);
}
extern "C" {
    pub fn GC_get_time_limit() -> ::std::os::raw::c_ulong;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GC_timeval_s {
    pub tv_ms: ::std::os::raw::c_ulong,
    pub tv_nsec: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_GC_timeval_s() {
    assert_eq!(
        ::std::mem::size_of::<GC_timeval_s>(),
        8usize,
        concat!("Size of: ", stringify!(GC_timeval_s))
    );
    assert_eq!(
        ::std::mem::align_of::<GC_timeval_s>(),
        4usize,
        concat!("Alignment of ", stringify!(GC_timeval_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GC_timeval_s>())).tv_ms as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_timeval_s),
            "::",
            stringify!(tv_ms)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GC_timeval_s>())).tv_nsec as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_timeval_s),
            "::",
            stringify!(tv_nsec)
        )
    );
}
extern "C" {
    pub fn GC_set_time_limit_tv(arg1: GC_timeval_s);
}
extern "C" {
    pub fn GC_get_time_limit_tv() -> GC_timeval_s;
}
extern "C" {
    pub fn GC_start_performance_measurement();
}
extern "C" {
    pub fn GC_get_full_gc_total_time() -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn GC_set_pages_executable(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_pages_executable() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_min_bytes_allocd(arg1: usize);
}
extern "C" {
    pub fn GC_get_min_bytes_allocd() -> usize;
}
extern "C" {
    pub fn GC_set_rate(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_rate() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_max_prior_attempts(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_max_prior_attempts() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_handle_fork(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_atfork_prepare();
}
extern "C" {
    pub fn GC_atfork_parent();
}
extern "C" {
    pub fn GC_atfork_child();
}
extern "C" {
    pub fn GC_init();
}
extern "C" {
    pub fn GC_is_init_called() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_deinit();
}
extern "C" {
    pub fn GC_malloc(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_malloc_atomic(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_strdup(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn GC_strndup(
        arg1: *const ::std::os::raw::c_char,
        arg2: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn GC_malloc_uncollectable(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_malloc_stubborn(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_memalign(arg1: usize, arg2: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_posix_memalign(
        arg1: *mut *mut ::std::os::raw::c_void,
        arg2: usize,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_free(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn GC_change_stubborn(arg1: *const ::std::os::raw::c_void);
}
extern "C" {
    pub fn GC_end_stubborn_change(arg1: *const ::std::os::raw::c_void);
}
extern "C" {
    pub fn GC_base(arg1: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_is_heap_ptr(arg1: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_size(arg1: *const ::std::os::raw::c_void) -> usize;
}
extern "C" {
    pub fn GC_realloc(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_expand_hp(arg1: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_max_heap_size(arg1: GC_word);
}
extern "C" {
    pub fn GC_exclude_static_roots(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GC_clear_roots();
}
extern "C" {
    pub fn GC_add_roots(arg1: *mut ::std::os::raw::c_void, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn GC_remove_roots(arg1: *mut ::std::os::raw::c_void, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn GC_register_displacement(arg1: usize);
}
extern "C" {
    pub fn GC_debug_register_displacement(arg1: usize);
}
extern "C" {
    pub fn GC_gcollect();
}
extern "C" {
    pub fn GC_gcollect_and_unmap();
}
pub type GC_stop_func = ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>;
extern "C" {
    pub fn GC_try_to_collect(arg1: GC_stop_func) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_set_stop_func(arg1: GC_stop_func);
}
extern "C" {
    pub fn GC_get_stop_func() -> GC_stop_func;
}
extern "C" {
    pub fn GC_get_heap_size() -> usize;
}
extern "C" {
    pub fn GC_get_free_bytes() -> usize;
}
extern "C" {
    pub fn GC_get_unmapped_bytes() -> usize;
}
extern "C" {
    pub fn GC_get_bytes_since_gc() -> usize;
}
extern "C" {
    pub fn GC_get_expl_freed_bytes_since_gc() -> usize;
}
extern "C" {
    pub fn GC_get_total_bytes() -> usize;
}
extern "C" {
    pub fn GC_get_heap_usage_safe(
        arg1: *mut GC_word,
        arg2: *mut GC_word,
        arg3: *mut GC_word,
        arg4: *mut GC_word,
        arg5: *mut GC_word,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GC_prof_stats_s {
    pub heapsize_full: GC_word,
    pub free_bytes_full: GC_word,
    pub unmapped_bytes: GC_word,
    pub bytes_allocd_since_gc: GC_word,
    pub allocd_bytes_before_gc: GC_word,
    pub non_gc_bytes: GC_word,
    pub gc_no: GC_word,
    pub markers_m1: GC_word,
    pub bytes_reclaimed_since_gc: GC_word,
    pub reclaimed_bytes_before_gc: GC_word,
    pub expl_freed_bytes_since_gc: GC_word,
}
#[test]
fn bindgen_test_layout_GC_prof_stats_s() {
    assert_eq!(
        ::std::mem::size_of::<GC_prof_stats_s>(),
        88usize,
        concat!("Size of: ", stringify!(GC_prof_stats_s))
    );
    assert_eq!(
        ::std::mem::align_of::<GC_prof_stats_s>(),
        8usize,
        concat!("Alignment of ", stringify!(GC_prof_stats_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GC_prof_stats_s>())).heapsize_full as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(heapsize_full)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GC_prof_stats_s>())).free_bytes_full as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(free_bytes_full)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GC_prof_stats_s>())).unmapped_bytes as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(unmapped_bytes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GC_prof_stats_s>())).bytes_allocd_since_gc as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(bytes_allocd_since_gc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GC_prof_stats_s>())).allocd_bytes_before_gc as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(allocd_bytes_before_gc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GC_prof_stats_s>())).non_gc_bytes as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(non_gc_bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GC_prof_stats_s>())).gc_no as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(gc_no)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GC_prof_stats_s>())).markers_m1 as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(markers_m1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GC_prof_stats_s>())).bytes_reclaimed_since_gc as *const _
                as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(bytes_reclaimed_since_gc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GC_prof_stats_s>())).reclaimed_bytes_before_gc as *const _
                as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(reclaimed_bytes_before_gc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GC_prof_stats_s>())).expl_freed_bytes_since_gc as *const _
                as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(expl_freed_bytes_since_gc)
        )
    );
}
extern "C" {
    pub fn GC_get_prof_stats(arg1: *mut GC_prof_stats_s, arg2: usize) -> usize;
}
extern "C" {
    pub fn GC_get_size_map_at(i: ::std::os::raw::c_int) -> usize;
}
extern "C" {
    pub fn GC_get_memory_use() -> usize;
}
extern "C" {
    pub fn GC_disable();
}
extern "C" {
    pub fn GC_is_disabled() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_enable();
}
extern "C" {
    pub fn GC_set_manual_vdb_allowed(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_manual_vdb_allowed() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_enable_incremental();
}
extern "C" {
    pub fn GC_is_incremental_mode() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_incremental_protection_needs() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_collect_a_little() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_malloc_ignore_off_page(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_malloc_atomic_ignore_off_page(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_malloc_atomic_uncollectable(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_debug_malloc_atomic_uncollectable(
        arg1: usize,
        s: *const ::std::os::raw::c_char,
        i: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_debug_malloc(
        arg1: usize,
        s: *const ::std::os::raw::c_char,
        i: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_debug_malloc_atomic(
        arg1: usize,
        s: *const ::std::os::raw::c_char,
        i: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_debug_strdup(
        arg1: *const ::std::os::raw::c_char,
        s: *const ::std::os::raw::c_char,
        i: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn GC_debug_strndup(
        arg1: *const ::std::os::raw::c_char,
        arg2: usize,
        s: *const ::std::os::raw::c_char,
        i: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn GC_debug_malloc_uncollectable(
        arg1: usize,
        s: *const ::std::os::raw::c_char,
        i: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_debug_malloc_stubborn(
        arg1: usize,
        s: *const ::std::os::raw::c_char,
        i: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_debug_malloc_ignore_off_page(
        arg1: usize,
        s: *const ::std::os::raw::c_char,
        i: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_debug_malloc_atomic_ignore_off_page(
        arg1: usize,
        s: *const ::std::os::raw::c_char,
        i: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_debug_free(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn GC_debug_realloc(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
        s: *const ::std::os::raw::c_char,
        i: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_debug_change_stubborn(arg1: *const ::std::os::raw::c_void);
}
extern "C" {
    pub fn GC_debug_end_stubborn_change(arg1: *const ::std::os::raw::c_void);
}
extern "C" {
    pub fn GC_debug_malloc_replacement(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_debug_realloc_replacement(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
    ) -> *mut ::std::os::raw::c_void;
}
pub type GC_finalization_proc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn GC_register_finalizer(
        arg1: *mut ::std::os::raw::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GC_debug_register_finalizer(
        arg1: *mut ::std::os::raw::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GC_register_finalizer_ignore_self(
        arg1: *mut ::std::os::raw::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GC_debug_register_finalizer_ignore_self(
        arg1: *mut ::std::os::raw::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GC_register_finalizer_no_order(
        arg1: *mut ::std::os::raw::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GC_debug_register_finalizer_no_order(
        arg1: *mut ::std::os::raw::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GC_register_finalizer_unreachable(
        arg1: *mut ::std::os::raw::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GC_debug_register_finalizer_unreachable(
        arg1: *mut ::std::os::raw::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GC_register_disappearing_link(
        arg1: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_general_register_disappearing_link(
        arg1: *mut *mut ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_move_disappearing_link(
        arg1: *mut *mut ::std::os::raw::c_void,
        arg2: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_unregister_disappearing_link(
        arg1: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_register_long_link(
        arg1: *mut *mut ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_move_long_link(
        arg1: *mut *mut ::std::os::raw::c_void,
        arg2: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_unregister_long_link(arg1: *mut *mut ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
}
pub const GC_ToggleRefStatus_GC_TOGGLE_REF_DROP: GC_ToggleRefStatus = 0;
pub const GC_ToggleRefStatus_GC_TOGGLE_REF_STRONG: GC_ToggleRefStatus = 1;
pub const GC_ToggleRefStatus_GC_TOGGLE_REF_WEAK: GC_ToggleRefStatus = 2;
pub type GC_ToggleRefStatus = i32;
pub type GC_toggleref_func = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> GC_ToggleRefStatus,
>;
extern "C" {
    pub fn GC_set_toggleref_func(arg1: GC_toggleref_func);
}
extern "C" {
    pub fn GC_get_toggleref_func() -> GC_toggleref_func;
}
extern "C" {
    pub fn GC_toggleref_add(
        arg1: *mut ::std::os::raw::c_void,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type GC_await_finalize_proc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn GC_set_await_finalize_proc(arg1: GC_await_finalize_proc);
}
extern "C" {
    pub fn GC_get_await_finalize_proc() -> GC_await_finalize_proc;
}
extern "C" {
    pub fn GC_should_invoke_finalizers() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_invoke_finalizers() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_noop1(arg1: GC_word);
}
pub type GC_warn_proc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_char, arg2: GC_word)>;
extern "C" {
    pub fn GC_set_warn_proc(arg1: GC_warn_proc);
}
extern "C" {
    pub fn GC_get_warn_proc() -> GC_warn_proc;
}
extern "C" {
    pub fn GC_ignore_warn_proc(arg1: *mut ::std::os::raw::c_char, arg2: GC_word);
}
extern "C" {
    pub fn GC_set_log_fd(arg1: ::std::os::raw::c_int);
}
pub type GC_abort_func =
    ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>;
extern "C" {
    pub fn GC_set_abort_func(arg1: GC_abort_func);
}
extern "C" {
    pub fn GC_get_abort_func() -> GC_abort_func;
}
extern "C" {
    pub fn GC_abort_on_oom();
}
pub type GC_hidden_pointer = GC_word;
pub type GC_fn_type = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
>;
extern "C" {
    pub fn GC_call_with_alloc_lock(
        arg1: GC_fn_type,
        arg2: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GC_stack_base {
    pub mem_base: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_GC_stack_base() {
    assert_eq!(
        ::std::mem::size_of::<GC_stack_base>(),
        8usize,
        concat!("Size of: ", stringify!(GC_stack_base))
    );
    assert_eq!(
        ::std::mem::align_of::<GC_stack_base>(),
        8usize,
        concat!("Alignment of ", stringify!(GC_stack_base))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GC_stack_base>())).mem_base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_stack_base),
            "::",
            stringify!(mem_base)
        )
    );
}
pub type GC_stack_base_func = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut GC_stack_base,
        arg2: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void,
>;
extern "C" {
    pub fn GC_call_with_stack_base(
        arg1: GC_stack_base_func,
        arg2: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_do_blocking(
        arg1: GC_fn_type,
        arg2: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_call_with_gc_active(
        arg1: GC_fn_type,
        arg2: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_get_stack_base(arg1: *mut GC_stack_base) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_get_my_stackbottom(arg1: *mut GC_stack_base) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_set_stackbottom(arg1: *mut ::std::os::raw::c_void, arg2: *const GC_stack_base);
}
extern "C" {
    pub fn GC_same_obj(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_pre_incr(
        arg1: *mut *mut ::std::os::raw::c_void,
        arg2: isize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_post_incr(
        arg1: *mut *mut ::std::os::raw::c_void,
        arg2: isize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_is_visible(arg1: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_is_valid_displacement(
        arg1: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GC_dump();
}
extern "C" {
    pub fn GC_dump_named(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GC_dump_regions();
}
extern "C" {
    pub fn GC_dump_finalization();
}
extern "C" {
    pub fn GC_ptr_store_and_dirty(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GC_debug_ptr_store_and_dirty(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub static mut GC_same_obj_print_proc: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: *mut ::std::os::raw::c_void),
    >;
}
extern "C" {
    pub static mut GC_is_valid_displacement_print_proc:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
}
extern "C" {
    pub static mut GC_is_visible_print_proc:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
}
extern "C" {
    pub fn GC_malloc_many(arg1: usize) -> *mut ::std::os::raw::c_void;
}
pub type GC_has_static_roots_func = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_void,
        arg3: usize,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn GC_register_has_static_roots_callback(arg1: GC_has_static_roots_func);
}
extern "C" {
    pub fn GC_set_force_unmap_on_gcollect(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GC_get_force_unmap_on_gcollect() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GC_win32_free_heap();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}
