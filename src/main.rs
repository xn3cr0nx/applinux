#[allow(unused_imports)]
use std::path::Path;
use std::fs;
#[allow(unused_imports)]
use std::fs::File;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;

fn main() {
  let name : &'static str = "applinux";

  let matches = App::new(name)
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
      Arg::with_name("destination")
      .long("dest")
      .help("app destination path")
      .takes_value(true)
      // .validator(applinux::is_dir)
      .global(true)
    )
    .arg(
      Arg::with_name("remove")
      .long("rm")
      .help("remove source files, binary and icon")
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
    )
    .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
      println!("{} {} running", name.blue().bold(), subcommand.green().blink());
    }

    match matches.subcommand() {
      ("new", Some(new_matches)) => {
        match new_match(name, &matches, new_matches) {
          Ok(s) => println!("Package successfully created: {}", s),
          Err(s) => println!("{} {}", "error:".red().bold(), s)
        };
      }
      _ => unreachable!()
    };
}

fn new_match(name: &'static str , m: & ArgMatches, sub: & ArgMatches) -> Result<String, String> {
  // println!("{:?}", sub.value_of("binary").unwrap());
  // println!("{:?}", m.value_of("destination").unwrap());

  if sub.value_of("icon").is_some() {
    println!("{:?}", sub.value_of("icon").unwrap());
  }

  let path = Path::new(m.value_of("destination").unwrap_or_else(|| "/usr/local")).join(name);
  match fs::create_dir(&path) {
    Ok(_s) => {
      println!("{}", "Folder created".green());
      let bin = sub.value_of("binary").unwrap();

      let desktop_file = &path.join(format!("{}.desktop", &bin));
      if fs::copy(Path::new("./examples.desktop"), &desktop_file).is_ok() {
        let data = fs::read_to_string(&desktop_file).unwrap();

        // Run the replace operation in memory
        let mut new_data = data.replace("<name>", &bin);
        new_data = new_data.replace("<exec>", &path.join(&bin).to_string_lossy());
        // new_data = new_data.replace("<icon>", &path.join(&icon).to_string_lossy());

        fs::write(&desktop_file, new_data.as_bytes()).unwrap();
      } else {
        return Err(String::from("cannot copy desktop file"))
      }

      match fs::copy(&bin, &path.join(&bin)) {
        Ok(_s) => {
          println!("{}", "Binary copied".green());
          if sub.value_of("icon").is_some() { 
            let icon = sub.value_of("icon").unwrap();
            match fs::copy(&icon, &path.join(&icon)) {
              Ok(_s) => {
                Ok(String::from("test ok"))
              },
              Err(s) => Err(s.to_string())
            }
          } else {
            println!("Missing icon");
            Ok(String::from("test ok"))
          }
        },
        Err(s) => Err(s.to_string())
      } 
    },
    Err(s) => Err(s.to_string())
  }
}
