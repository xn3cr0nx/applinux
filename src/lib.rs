#[allow(unused_imports)]
use std::path::Path;

pub fn is_dir(val: String) -> Result<(), String> {
  let path = Path::new(&val);
  if path.is_dir() {
    if path.is_absolute() {
      Ok(())
    } else {
      Err(String::from("path should be absolute"))
    }
  } else {
      Err(String::from("the argument is not a directory"))
  }
}


pub fn is_file(val: String) -> Result<(), String> {
    if Path::new(&val).is_file() {
        Ok(())
    } else {
        Err(String::from("the argument is not a file"))
    }
}

pub fn is_icon(val: String) -> Result<(), String> {
  if val.ends_with(".ico") || val.ends_with(".png") || val.ends_with(".jpeg") {
    if is_file(val).is_ok() {
      Ok(())
    } else {
      Err(String::from("icon file is missing")) 
    }
  } else {
    Err(String::from("the icon file extension should be ico, png or jpeg"))
  }
}

pub fn get_desktop_template() -> String {
  String::from("[Desktop Entry]
Version=1.0
Type=Application
Name=<name>
Comment=<comment>
Exec=<exec>
Icon=<icon>
NoDisplay=true
Terminal=false")
}

#[test]
fn check_is_path() {
  let path : String = String::from("/tmp"); 
  assert_eq!(is_dir(path).is_ok(), true);
  let wrong_path : String = String::from("/path"); 
  assert_eq!(is_dir(wrong_path).is_ok(), false);
  let not_absolute_path : String = String::from("tmp"); 
  assert_eq!(is_dir(not_absolute_path).is_ok(), false)
}

#[test]
fn check_is_file() {
  let file : String = String::from("Cargo.toml"); 
  assert_eq!(is_file(file).is_ok(), true);
  let wrong_file : String = String::from("path"); 
  assert_eq!(is_file(wrong_file).is_ok(), false)
}

#[test]
fn check_is_icon() {
  let icon : String = String::from("Cargo.toml");
  assert_eq!(is_icon(icon).is_ok(), false);
}
