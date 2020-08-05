use clap::{App, Arg, Clap};
use std::num::ParseFloatError;
use std::str::FromStr;

fn run() {
    // let options = Options::parse();
}

#[derive(Clap)]
#[clap(version = "0.1", author = "Vitaly Kravchenko <vitalyx@gmail.com>")]
struct Options {
    #[clap(
        long,
        value_name = "x0,x1,y0,y1",
        default_value = "0,0,800,600",
        about = "Specify an image crop window."
    )]
    crop_window: CropWindow,
    #[clap(
        long,
        default_value = "0",
        about = "Use specified number of threads for rendering."
    )]
    thread_count: i32,
    #[clap(
        long,
        about = "Automatically reduce a number of quality settings
to render more quickly."
    )]
    quick_render: bool,
    #[clap(long, about = "Suppress all text output other than error messages.")]
    quiet: bool,
    #[clap(
        short,
        long = "outfile",
        default_value = "out.png",
        about = "Write the final image to the given filename."
    )]
    image_file: String,
}

#[derive(Debug)]
struct CropWindow(((f32, f32), (f32, f32)));

impl FromStr for CropWindow {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_coords: Vec<&str> = s.split(',').collect();
        let x0 = str_coords[0].parse::<f32>()?;
        let x1 = str_coords[1].parse::<f32>()?;
        let y0 = str_coords[2].parse::<f32>()?;
        let y1 = str_coords[3].parse::<f32>()?;
        Ok(CropWindow(((x0, x1), (y0, y1))))
    }
}