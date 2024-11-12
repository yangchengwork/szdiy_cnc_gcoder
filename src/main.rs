use std::env;
use tokio;
use tokio::sync::mpsc;
use anyhow::Result;
use std::{fs::File, io::{copy, Cursor}};

use std::io::Read;
use std::path::Path;
use vtracer::Config;

use svg2gcode::{
    svg2program, ConversionConfig, ConversionOptions, Machine, MachineConfig, Settings, SupportedFunctionality, Version
};
use g_code::emit::{format_gcode_io, FormatOptions};

use roxmltree::ParsingOptions;

use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    object: String,

    // Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    // Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,

    // #[command(subcommand)]
    // command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<()> {

    let cli = Cli::parse();
    // let mut words = "cat";
    // if let Some(words) = cli.object.as_deref() {
    //     println!("Yours: {words}~~");
    // }
    let words = cli.object;

    let jpeg_path =Path::new("foo.jpeg");
    let svg_path =Path::new("foo.svg");
    let gcode_path =Path::new("foo.gcode");
    // println!("Hello, world!");
    let url = "https://api-inference.huggingface.co/models/black-forest-labs/FLUX.1-dev";
    // let args: Vec<String> = env::args().collect();
    // let url = args[1].clone();
    // let words = args[1].clone();
    let prompt = format!("Minimalistic outline illustration of {words} stick figure, simple yet elegant, blake and white, only outlines");

    let mut file = File::create(jpeg_path)?;
    // let n = file.write(words.as_bytes()).await?;

    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        let res = gcoder::sentiment(&url, &prompt).await.unwrap();
        tx.send(res).await.unwrap();
    });

    while let Some(response) = rx.recv().await {
        // let sentiment: Lables = serde_json::from_str(&message)?;

        // let n = file.write(message.as_bytes()).await?;

    // Create a cursor that wraps the response body
    let mut content =  Cursor::new(response.bytes().await?);
    // Copy the content from the cursor to the file
    copy(&mut content, &mut file)?;
    }

    println!("jpeg to svg.");
    let jpeg_svg_config =  Config::default();
    // let svg = vtracer::jpeg_to_svg("foo.jpeg", jpeg_svg_config).await?;
    let result = vtracer::convert_image_to_svg(jpeg_path, svg_path, jpeg_svg_config);
    match result {
        Ok(()) => {
            println!("Conversion successful.");
        }
        Err(msg) => {
            panic!("Conversion failed with error message: {}", msg);
        }
    }

    let svg_gcode_config = ConversionConfig::default();
    let svg_gcode_option = ConversionOptions::default();
    let machine_config = MachineConfig::default();

        let mut f = File::open(svg_path)?;
        let len = f.metadata()?.len();
        let mut input = String::with_capacity(len as usize + 1);
        f.read_to_string(&mut input)?;

    let document = roxmltree::Document::parse_with_options(
        &input,
        ParsingOptions {
            allow_dtd: true,
            ..Default::default()
        },
    )?;

    let machine = Machine::new(machine_config.supported_functionality, None, None, None, None);
    println!("svg to program.");

    let program = svg2program(&document, &svg_gcode_config, svg_gcode_option, machine);
    println!("program to gcode.");
    format_gcode_io(
        &program,
        FormatOptions {
            line_numbers: false,
            checksums: false,
            ..Default::default()
        },
        File::create(gcode_path)?,
    )?;
    println!("Done.");
    Ok(())
}
