#[allow(unused_imports)]
use std::path::Path;
use std::fs;
#[allow(unused_imports)]
use std::fs::File;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;

fn main() {
  let matches = App::new("applinux")
    .about("A tool to patch your linux desktop applications")
    .version("0.1")
    .author("Patrick Jusic <patrick.jusic@protonmail.com>")
    .arg(
      Arg::with_name("debug")
      .long("debug")
      .short("d")
      .help("debug output")
      .takes_value(false)  
      .global(true)
    )
    .arg(
      Arg::with_name("name")
      .long("name")
      .short("n")
      .help("specify application name")
      .takes_value(true)
      .global(true)
    )
    .arg(
      Arg::with_name("comment")
      .long("comment")
      .short("c")
      .help("specify application comment")
      .takes_value(true)
      .global(true)
    )
    .arg(
      Arg::with_name("destination")
      .long("dest")
      .help("specify app destination path")
      .takes_value(true)
      .validator(applinux::is_dir)
      .global(true)
    )
    .arg(
      Arg::with_name("remove")
      .long("rm")
      .help("remove source files, binary and icon")
      .takes_value(false)
      .global(true)
    )
    .subcommand(
      SubCommand::with_name("new")
        .about("create new app")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
          Arg::with_name("binary")
            .short("b")
            .long("bin")
            .help("binary or appimage path")
            .required(true)
            .takes_value(true)
            .validator(applinux::is_file)
        )
        .arg(
          Arg::with_name("icon")
          .short("i")
          .long("icon")
          .help("icon path")
          .takes_value(true)
          .validator(applinux::is_icon)
        )
    ).get_matches();

    println!("{}\n", "Applinux".blue().bold().blink());

    match matches.subcommand() {
      ("new", Some(new_matches)) => {
        match new_match(&matches, new_matches) {
          Ok(_s) => println!("{}", "Package successfully created".green().bold()),
          Err(s) => println!("{} {}", "error:".red().bold(), s)
        };
      },
      ("", None) => {
        println!("{}", matches.usage());
      },
      _ => unreachable!()
    };
}

/* 
 * new_match creates a new package based on passed bin.
 * First the package folder is created in specified destination, /usr/local otherwise.
 * Inside the package the specified binary is copied and, if specified, the icon.
 * Eventually the desktop file is created to /usr/share/applications. Placeholders for name, exec and icon are replaced.
 * If --rm flag is specified both bin and icon sources ar removed.
 */
fn new_match(m: & ArgMatches, sub: & ArgMatches) -> Result<String, String> {
  let mut bin = Path::new(sub.value_of("binary").unwrap());
  if bin.starts_with("./") {
    bin = bin.strip_prefix("./").unwrap();
  } 
  let filename = bin.file_name().unwrap().to_str().unwrap();

  let mut icon = Path::new(sub.value_of("icon").unwrap_or_else(|| "None"));
  if icon.starts_with("./") {
    icon = icon.strip_prefix("./").unwrap();
  } 
  let fileicon = icon.file_name().unwrap().to_str().unwrap();

  let name = m.value_of("name").unwrap_or_else(|| &filename); 
  let comment = m.value_of("comment").unwrap_or_else(|| "None"); 
  let path = Path::new(m.value_of("destination").unwrap_or_else(|| "/usr/local")).join(&name);
  let rm = m.is_present("remove");

  println!("Creating package {} ({})", name.blue(), comment);
  println!("Binary location: {}", bin.to_str().unwrap().blue());
  println!("Icon location: {}", icon.to_str().unwrap().blue());
  println!("removing source: {}", rm);

  print!("Creating package directory... ");
  match fs::create_dir(&path) {
    Ok(_s) => {
      println!("{}", "directory created".green());

      print!("Copying binary to package directory... ");
      match fs::copy(&bin, &path.join(&filename)) {
        Ok(_s) => {
          println!("{}", "binary copied".green());

          if rm {
            print!("Removing source binary... ");
            match fs::remove_file(&bin) {
              Ok(_s) => println!("{}", "source binary removed".green()),
              Err(s) => println!("{}: {}", "error cannot remove bin source:".red(), s.to_string())
            }
          }

          if fileicon != "None" { 
            print!("Copying icon to package directory... ");
            match fs::copy(&icon, &path.join(&fileicon)) {
              Ok(_s) => {
                println!("{}", "icon copied".green());
                if rm {
                  print!("Removing source icon... ");
                  match fs::remove_file(&icon) {
                    Ok(_s) => println!("{}", "source icon removed".green()),
                    Err(s) => println!("{}: {}", "error cannot remove bin source:".red(), s.to_string())
                  }
                }
              },
              Err(s) => return Err(s.to_string())
            }
          }
        },
        Err(s) => return Err(s.to_string())
      } 

      print!("Creating desktop file... ");
      let desktop_file = format!("{}.desktop", &name);
      let desktop_path = Path::new("/usr/share/applications").join(&desktop_file);
      let data = applinux::get_desktop_template();
      let mut new_data = data.replace("<name>", &name);
      new_data = new_data.replace("<exec>", &path.join(&filename).to_string_lossy());
      if fileicon != "None" {
        new_data = new_data.replace("<icon>", &path.join(&fileicon).to_string_lossy());
      }
      if comment != "None" {
        new_data = new_data.replace("<comment>", comment);
      }
      fs::write(&desktop_path, new_data.as_bytes()).unwrap();
      println!("{}", "desktop file created".green());

      Ok(String::from(""))

    },
    Err(s) => Err(s.to_string())
  }
}
