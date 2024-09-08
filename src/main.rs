use core_graphics::display::{self, *};

use core_foundation::base::*;
use core_foundation::number::*;
use core_foundation::string::*;
use core_graphics::window;
use std::ffi::c_void;

fn main() {
  let w = get_window_infos();
  let num = w.len();

  println!("Number of windows: {}", num);

  for i in w.iter() {
    println!("{:?}", i);
  }
}

#[derive(Debug)]
struct WindowInfo {
  pub id: CGWindowID,
  pub name: String,
  pub bounds: CGRect,
}

fn get_window_infos() -> Vec<WindowInfo> {
  let mut win_infos = vec![];
  let wins = CGDisplay::window_list_info(
    display::kCGWindowListOptionAll, // Fetch all windows
    None,
  );
  if let Some(wins) = wins {
    for w in wins.iter() {
      let w: CFDictionary<*const c_void, *const c_void> =
        unsafe { CFDictionary::wrap_under_get_rule(*w as CFDictionaryRef) };

      // Get window owner name (application name)
      let name = match w.find(unsafe { window::kCGWindowOwnerName }.to_void()) {
        Some(owner) => unsafe { CFString::wrap_under_get_rule(*owner as CFStringRef) }.to_string(),
        None => continue, // Skip if no owner name is found
      };

      // Get window ID
      let id = w.get(unsafe { window::kCGWindowNumber }.to_void());
      let id = unsafe { CFNumber::wrap_under_get_rule(*id as CFNumberRef) }
        .to_i64()
        .unwrap() as CGWindowID;

      // Get window bounds
      let bounds = w.get(unsafe { window::kCGWindowBounds }.to_void());
      let bounds = unsafe { CFDictionary::wrap_under_get_rule(*bounds as CFDictionaryRef) };
      let bounds = CGRect::from_dict_representation(&bounds).unwrap();

      // Push window info to list
      win_infos.push(WindowInfo { id, name, bounds });
    }
  }
  win_infos
}
