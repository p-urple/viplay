use clap::Parser;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use rodio::cpal::{BufferSize, SAMPLE_RATE_48K};
use std::process;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::num::NonZero;

#[derive(Parser)]
#[command(name = "vp", author, version)]
struct Cli { 
    // path argument
    #[arg(required = false, value_name = "FILE")]
    files: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    // override the default configured buffer size
    // create sink from custom configuration
    let builder = DeviceSinkBuilder::from_default_device()
        .expect("Cannot create audio sink");
    let handle = builder
        .with_sample_rate(NonZero::new(SAMPLE_RATE_48K).unwrap())
        .with_buffer_size(BufferSize::Default)
        .open_stream().unwrap();
    println!("debug");
    // build a basic mixer player and a queue 
    let player = Player::connect_new(&handle.mixer());

    // extract later:
    // checks if no file is passed to exit gracefully
    if cli.files.is_empty() { 
        println!("Warrning: No audio files provided.");
        // later extraction: contextual error messages?
        println!("Usage: music-cli <FILE>...");
        process::exit(0);
    }

    // read the file paths and append to queue
    println!("Attempting to queue {} tracks...", cli.files.len());
    for path in cli.files {
        // try to open the file
        let file = BufReader::new(File::open(&path).unwrap());
        // decode the audio file into a source
        let source = Decoder::new(file).unwrap();

        player.append(source);
        println!("-> Added to queue: {}", path.display());
    }
    
    player.sleep_until_end();
}
