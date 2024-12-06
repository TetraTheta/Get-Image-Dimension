use clap::Parser;
use image::GenericImageView;
use std::{path::PathBuf, process};

#[derive(Parser)]
#[command(version, about = "Prints the dimensions of an image in 'width x height' format.", long_about = None)]
struct Args {
  file: Option<PathBuf>,
}

fn main() {
  let args = Args::parse();

  let file_path = match args.file {
    Some(path) => path,
    None => {
      eprintln!("\x1b[31mERROR\x1b[0m No file path is provided.");
      process::exit(1)
    }
  };

  let img = match image::open(&file_path) {
    Ok(image) => image,
    Err(_) => {
      eprintln!("\x1b[31mERROR\x1b[0m Could not open the image file or the file is not a valid image.");
      process::exit(1)
    }
  };

  let dimension = img.dimensions();
  println!("{} x {}", dimension.0, dimension.1);
}
