mod ui;
mod capture {
    pub mod video; 
}
use clap::Parser;
use std::{error::Error, fmt::format};
use ui::iced_ui::{self};
use capture::video::{self, video_capture};

#[derive(Parser, Debug)]
struct Args {
#[arg(short, long)]
    video: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.video {
        true => {
            println!("Captured frames: {:?}", video_capture::capture_video(30, 60));
            Ok(())
        }
        _ => {
            iced_ui::run().map_err(|e| Box::new(e) as Box<dyn Error>);
            Ok(())
        }
    }
}
