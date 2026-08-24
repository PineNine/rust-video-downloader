use std::process::{Command, Stdio};
use clap::Parser;
use::std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // grabs the first argument without a flag -> link
    link: String,

    ///name of file
    #[arg(short, long, default_value_t=String::from("nvdr-temp"))]
    output: String,

    /// convert to gif
    #[arg(short, long,)]
    gif: bool,

    /// scale to desired dimensions DEFAULT=720
    #[arg(short, long, default_value_t="720".to_string())]
    scale: String,

    /// keep video and gif
    #[arg(short, long,)]
    keep: bool,
}



impl Args {

    todo!("FINISH THIS MATCH grem")
    match fs::exists(&video_name) {
        Ok(true) => 
    }
    fn download(&self, video_name: &String){
        Command::new("yt-dlp")
            .arg("-t")
            .arg("mp4")
            .arg(&self.link)
            .arg("-o")
            .arg(&video_name)
            .stdout(Stdio::inherit())
            .output()
            .expect("failed to run lol");
    }

    fn convert(&self, video_name: &String) {
        let gif_name = format!("{}.gif", &self.output);
        // create palette for gif creation


        Command::new("ffmpeg")
            .arg("-i")
            .arg(&video_name)
            .arg("-filter_complex")
            .arg("[0:v] palettegen")
            .arg("nvdr-palette.png")
            .stdout(Stdio::inherit())
            .status()
            .expect("failed to create palette");
        
        // convert video file into gif using generated pallete
        Command::new("ffmpeg")
            .arg("-i")
            .arg(&video_name)
            .arg("-i")
            .arg("nvdr-palette.png")
            .arg("-filter_complex")
            .arg("[0:v][1:v] paletteuse")
            .arg(gif_name)
            .stdout(Stdio::inherit())
            .status()
            .expect("failed to create gif");
    }

}


fn main() {
    let args = Args::parse();

    let video_name = format!("{}.mp4", &args.output);

    args.download(&video_name);
    if let Args{gif: true, ..} = args {
        args.convert(&video_name);
    }
}

