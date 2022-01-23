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

/// Encode the given input image files
/// into a sfa file. sfa file stores the images
/// in PNG format only thus you might loose some
/// quality while saving in JPEG or lossfully
/// formated images. Thus using a lossless format
/// for input is recommended.
///
/// # Arguments
///
/// * `input_files` - It is a reference to a slice of `&str` objects.
/// * `output_file` - It is a file that is created and written to.
///   Regardless of file extension the output format is always SFA.
///   It accepts any Path-like object as `output_file` path.
///
/// # Errors
///
/// The errors are dynamic and are generated in one of the
/// following conditions.
///
/// * If the file can not be created due to some OS Error
/// * If buffered writer fails to write some information due
///   to some OS error.
/// * If provided input_files are not valid images or if there
///   is some error while reading them.
/// * If there are problems with writing the image in PNG format
///   to an in memory buffer.
///
/// # Examples
///
/// ```no_run
/// use sfa::encode;
///
/// let my_sprite = ["sp1.png", "sp2.png", "sp3.png"];
///
/// encode(&my_sprite, "sp.sfa");
/// ```
pub fn encode<T: AsRef<Path>>(
    input_files: &[&str],
    output_file: T,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::create(output_file)?;
    let mut file_writer = io::BufWriter::new(file);

    file_writer.write_all(b"SFA;")?;

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
        file_writer.write_all(format!("{}:{}:", x, temprorary_buffer.len()).as_bytes())?;
        file_writer.write_all(&temprorary_buffer)?;
    }
    file_writer.flush()?;

    Ok(())
}

/// Decodes an SFA file from disk and returns a result of dynamic Error
/// or the expected `HashMap<String, image::DynamicImage>`. The values
/// can then be converted for use with other graphics generating libraries.
///
/// Please refer to `sfa::decode_from_reader` for more insight because
/// this is a wrapper over that function.
///
/// # Arguments
///
/// * `file` - A path like object that refers to thte file to read for decoding. (`AsRef<Path>`)
///
/// # Examples
///
/// ```no_run
/// # use std::collections::HashMap;
/// use sfa::decode;
/// use image::DynamicImage;
///
/// let my_sprite: HashMap<String, DynamicImage> = decode("sp.sfa").unwrap();
///
/// let my_sprite_frame_1 = &my_sprite["sp1.png"];
/// ```
pub fn decode<P: AsRef<Path>>(
    file: P,
) -> Result<HashMap<String, image::DynamicImage>, Box<dyn std::error::Error>> {
    let file = fs::File::open(file)?;
    let mut file = io::BufReader::new(file);

    decode_from_reader(&mut file)
}

/// Decodes sfa file from a reader object that implements
/// the trait `std::io::Read`. It returns a in memory HashMap
/// with keys being `String` objects which are the original names
/// of files and the values are `image::DynamicImage` which are always
/// in PNG format. You might want to use this function for reading
/// from TcpStreams or some in memory buffer. It does not matter
/// if the stream is buffered or not because all the data is read
/// in one go. So for network streams the data will be downloaded
/// first and then processed.
///
/// Most of the times you only want to read from disk and is thus
/// recommended to use `sfa::decode` instead for convienience.
///
/// # Arguments
///
/// * `reader` - An object that implements the trait `io::Read`.
///
/// # Errors
///
/// Errors are dynamic and can be returned in either one of these
/// situations.
///
/// * Reading from the reader was unsuccessful (Maybe due to timeout).
/// * The file does not comply with the sfa format.
///
/// # Examples
///
/// ```no_run
/// # use std::collections::HashMap;
/// # use std::fs;
/// use std::io::Read;
/// use image::DynamicImage;
/// use sfa::decode_from_reader;
///
/// let mut my_sprite_sfa_file = fs::File::open("sp1.sfa").unwrap();
///
/// let my_sprite: HashMap<String, DynamicImage> = decode_from_reader(&mut my_sprite_sfa_file).unwrap();
/// ```
pub fn decode_from_reader<F: Read>(
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

    while let Some(b) = consuming_iterator.next() {
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
            let im_contents = image::load_from_memory_with_format(&im_contents, ImageFormat::Png)?;
            results.insert(name.to_owned(), im_contents);
        }
    }

    Ok(results)
}

/// Error for handling custom errors by this
/// crate. This error is only returned when this
/// crate encounter's SFA format specific errors.
///
/// The feild contained is `s` which is a owned `String`
/// that describes the error.
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
