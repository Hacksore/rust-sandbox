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

  let allowed_app = vec!["discord", "wezterm"];
  for window in w.iter() {
    if !allowed_app.contains(&window.name.to_lowercase().as_str()) {
      continue;
    }

    println!(
      "[level: {:?} | owner_pid {:?}] {:?}, {:?}, {:?}",
      window.level, window.owner_pid, window.id, window.name, window.bounds
    );
  }
}

#[derive(Debug)]
struct WindowInfo {
  pub id: CGWindowID,
  pub owner_pid: i32,
  pub name: String,
  pub level: CGWindowLevel,
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

      // Get the window level
      let level = w.get(unsafe { window::kCGWindowLayer }.to_void());
      let level = unsafe { CFNumber::wrap_under_get_rule(*level as CFNumberRef) }
        .to_i32()
        .unwrap();

      // Get window order in the list so we know which apps are on top
      let owner_pid = w.get(unsafe { window::kCGWindowOwnerPID }.to_void());
      let owner_pid = unsafe { CFNumber::wrap_under_get_rule(*owner_pid as CFNumberRef) }
        .to_i32()
        .unwrap();

      // Get window bounds
      let bounds = w.get(unsafe { window::kCGWindowBounds }.to_void());
      let bounds = unsafe { CFDictionary::wrap_under_get_rule(*bounds as CFDictionaryRef) };
      let bounds = CGRect::from_dict_representation(&bounds).unwrap();

      // Push window info to list
      win_infos.push(WindowInfo {
        id,
        name,
        owner_pid,
        level,
        bounds,
      });
    }
  }
  win_infos
}
