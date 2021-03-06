use std::ffi::CStr;
use std::os::raw::c_char;

pub fn deref_cstr(string_ptr: *const c_char) -> Option<String> {
    let string_ref = unsafe { string_ptr.as_ref() };
    let string_cstr = match string_ref {
        Some(ptr) => unsafe { CStr::from_ptr(ptr).to_str().ok() },
        None => None,
    };
    return string_cstr.map(|s| s.to_owned());
}
