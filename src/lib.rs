//! SFA stands for Single File Assets. Although
//! it says 'Assets', It only really supports images
//! You can use this library to encode and decode
//! such files for, say animatable sprites. It converts
//! the given images to PNG for storage.

#![allow(dead_code)]

use image::{ImageFormat, ImageOutputFormat};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn encode<T: AsRef<Path>>(
    input_files: &[&str],
    output_file: T,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::create(output_file)?;
    let mut file_writer = io::BufWriter::new(file);

    file_writer.write(b"SFA;")?;

    for x in input_files {
        let im = image::open(*x)?;

        let mut temprorary_buffer: Vec<u8> = vec![];
        let mut temprorary_buffer_file = io::Cursor::new(&mut temprorary_buffer);
        let mut temprorary_buffer_file_writer = io::BufWriter::new(&mut temprorary_buffer_file);

        // Write data in PNG format to a Vec<u8>
        im.write_to(&mut temprorary_buffer_file_writer, ImageOutputFormat::Png)?;

        // Keep the memory signature as low as possible so drop all unused things
        drop(temprorary_buffer_file_writer);
        drop(temprorary_buffer_file);
        drop(im);

        // Write the size of the data as well as name of the file with the data itself
        file_writer.write(format!("{}:{}:", x, temprorary_buffer.len()).as_bytes())?;
        file_writer.write(&temprorary_buffer)?;
    }
    file_writer.flush()?;

    Ok(())
}

pub fn decode(
    file: &str,
) -> Result<HashMap<String, image::DynamicImage>, Box<dyn std::error::Error>> {
    let file = fs::File::open(file)?;
    let mut file = io::BufReader::new(file);

    decode_from_buffer(&mut file)
}

pub fn decode_from_buffer<F: Read>(
    reader: &mut F,
) -> Result<HashMap<String, image::DynamicImage>, Box<dyn std::error::Error>> {
    let mut buffer: Vec<u8> = vec![];
    reader.read_to_end(&mut buffer)?;

    let mut results: HashMap<String, image::DynamicImage> = HashMap::new();
    let mut consuming_iterator = buffer.into_iter();
    let mut contents: String = String::new();

    'parseloop_magic: loop {
        match consuming_iterator.next() {
            Some(b) => {
                contents.push(b as char);
                if contents == "SFA;" {
                    break 'parseloop_magic;
                } else if contents.len() >= 4 {
                    return Err(Error::new(String::from(
                        "Magic Text Identifier not found after parsing 4 letters of the file",
                    ))
                    .into());
                }
            }
            None => {
                return Err(Error::new(String::from(
                    "Reached EOF before the magic string was seen.",
                ))
                .into())
            }
        }
    }

    contents = String::new();
    let mut name = String::new();
    let mut size: usize;
    let mut cmode = "n";

    loop {
        match consuming_iterator.next() {
            Some(b) => {
                contents.push(b as char);
                if b as char == ':' && cmode == "n" {
                    name = String::from(&contents[0..contents.len() - 1]);
                    contents = String::new();
                    cmode = "s";
                } else if b as char == ':' && cmode == "s" {
                    size = String::from(&contents[0..contents.len() - 1]).parse()?;
                    contents = String::new();
                    cmode = "n";
                    let mut im_contents: Vec<u8> = Vec::with_capacity(size);
                    for _ in 0..size {
                        match consuming_iterator.next() {
                            Some(b) => {
                                im_contents.push(b);
                            }
                            None => {
                                return Err(Error::new(String::from(
                                    "Reached EOF before all file content was retrieved",
                                ))
                                .into());
                            }
                        }
                    }
                    let im_contents =
                        image::load_from_memory_with_format(&im_contents, ImageFormat::Png)?;
                    results.insert(name.to_owned(), im_contents);
                }
            }
            None => break,
        }
    }

    Ok(results)
}

#[derive(Debug)]
struct Error {
    s: String,
}

impl Error {
    fn new(s: String) -> Error {
        Error { s }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.s)
    }
}

impl std::error::Error for Error {}
