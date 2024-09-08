use core_graphics::display::*;
use core_graphics::display::{
  kCGNullWindowID, kCGWindowListOptionAll, CFArrayGetCount, CFArrayGetValueAtIndex,
  CGWindowListCopyWindowInfo,
};

use core_foundation::base::*;
use core_foundation::number::*;
use core_foundation::string::*;
use std::ffi::{c_void, CStr};

fn main() {
  unsafe {
    let window_list = CGWindowListCopyWindowInfo(kCGWindowListOptionAll, kCGNullWindowID);
    let n_windows = CFArrayGetCount(window_list);

    for i in 0..n_windows {
      let window = CFArrayGetValueAtIndex(window_list, i) as CFDictionaryRef;
      let name = get_window_name(window);
      let bounds = get_window_bounds(window);
      // println!("{:?}, {:?}", name, bounds)
    }

    CFRelease(window_list as CFTypeRef);
  }
}

// https://stackoverflow.com/a/60140186
fn get_window_name(dict_ref: CFDictionaryRef) -> Option<String> {
  let key = CFString::new("kCGWindowOwnerName");
  let mut value: *const c_void = std::ptr::null();

  if unsafe { CFDictionaryGetValueIfPresent(dict_ref, key.to_void(), &mut value) != 0 } {
    let cf_ref = value as CFStringRef;
    let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };
    if !c_ptr.is_null() {
      let c_result = unsafe { CStr::from_ptr(c_ptr) };
      return Some(String::from(c_result.to_str().unwrap()));
    }
  }
  return None;
}

fn get_window_bounds(dict_ref: CFDictionaryRef) {
  let key = CFString::new("kCGWindowBounds");
  let mut value: *const c_void = std::ptr::null();
  if unsafe { CFDictionaryGetValueIfPresent(dict_ref, key.to_void(), &mut value) != 0 } {
    let cf_ref = value as CFDictionaryRef;
  }
}

fn get_window_pid(dict_ref: CFDictionaryRef) -> Option<u64> {
  let key = CFString::new("kCGWindowOwnerPID");
  let mut value: *const c_void = std::ptr::null();

  if unsafe { CFDictionaryGetValueIfPresent(dict_ref, key.to_void(), &mut value) != 0 } {
    let cf_ref = value as CFNumberRef;
    let mut number: u64 = 0;
    let c_ptr = unsafe {
      CFNumberGetValue(
        cf_ref,
        kCFNumberSInt64Type,
        &mut number as *mut u64 as *mut c_void,
      )
    };
    if c_ptr {
      return Some(number);
    }
  }
  return None;
}
