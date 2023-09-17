// Extract the Preview JPEGs contained inside .ORF RAW files
// Place copy of exiftools next to exe file
// Place RAWs inside of RAW folder
// After running script, JPEGs will appear in JPEG folder
// Uses the tokio::task::spawn_blocking to use multiple cores

use std::path::Path;
use std::fs;
use std::process::Command;

fn exiftoolcall(command1: String, command2: String) {
  println!("{}", command1);
  Command::new("cmd").args(&["/C", &command1]).output().expect("failed");
  let cmd = Command::new("cmd").args(&["/C", &command2]).output().expect("failed");
  print!("{}", String::from_utf8_lossy(&cmd.stdout));
}

#[tokio::main]
async fn main() {
  if !Path::new("./RAW").exists() {
    println!("RAW foider does not exist");
    let res = fs::create_dir_all("./RAW");
    match res {
      Ok(()) => println!("RAW folder created, place RAWs in this folder"),
      Err(e) => println!("Error: {e:?}"),
    }
    std::process::exit(0);
  } else {
    if !Path::new("./RAW").is_dir() {
      println!("There is a file named RAW in the folder, needs to be removed");
    }
  }
  let res = fs::create_dir_all("./JPEG");
  match res {
    Ok(()) => (),
    Err(e) => println!("Error: {e:?}"),
  }

  let mut handles = vec![];
  let p = fs::read_dir("./RAW/").unwrap();
  for file in p {
    let file = file.unwrap().path();
    if !file.is_dir() {
      let mut string1: String = "exiftool.exe -b -PreviewImage ".to_string();
      string1.push_str(&file.display().to_string());
      string1.push_str(" > JPEG/");
      string1.push_str(&file.display().to_string()[6..file.display().to_string().len()-3]);
      string1.push_str("jpg");
      let mut string2: String = "exiftool.exe -overwrite_original -TagsFromFile ".to_string();
      string2.push_str(&file.display().to_string());
      string2.push_str(" -exif:all JPEG/");
      string2.push_str(&file.display().to_string()[6..file.display().to_string().len()-3]);
      string2.push_str("jpg");
      let handle = tokio::task::spawn_blocking(||exiftoolcall(string1, string2));
      handles.push(handle);
    }
  }
  futures::future::join_all(handles).await; // does the same thing as the for loop
  // for handle in handles {
  //   let _x = handle.await.unwrap();
  // }
  println!("Finished");
}
