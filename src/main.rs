mod lib;
use clap::{app_from_crate, arg, App};
use std::fs;
use std::path::Path;

fn main() {
    let app = app_from_crate!()
        .about("Creates SFA files or extract them")
        .subcommand(App::new("pack").about("Create a SFA archive")
            .arg(arg!(
                output_file: -o --outfile [FILENAME] "Output SFA file to save to"
            )).arg(arg!(
                [input_images] ... "Input Images to use. Can be any format but output will always be PNG."
            ))
        )
        .subcommand(App::new("unpack").about("Unpack SFA files")
            .arg(arg!(
                output_directory: -o --outdir [DIRECTORY] "Output Directory"
            )).arg(arg!(
                [input_file] "Input SFA file to process"
            ))
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("pack", sub_matches)) => {
            let input_images: Vec<&str> = sub_matches.values_of("input_images").unwrap().collect();
            let output_file = sub_matches.value_of("output_file").unwrap();

            lib::encode(&input_images, output_file)
                .expect("Unexpected error while encoding the images");
        }
        Some(("unpack", sub_matches)) => {
            let output_dir = Path::new(sub_matches.value_of("output_directory").unwrap());
            let input_file = sub_matches.value_of("input_file").unwrap();

            let extracted_data = lib::decode(input_file).unwrap();
            if !output_dir.exists() {
                fs::create_dir(output_dir).unwrap();
            }

            for (name, contents) in extracted_data.into_iter() {
                contents.save(output_dir.join(name)).unwrap();
            }
        }
        _ => (),
    }
}
