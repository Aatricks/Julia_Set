use clap::Parser;
use image::{ImageBuffer, Rgb};
use num::Complex;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

#[derive(Parser)]
#[command(name = "julia_set")]
#[command(about = "A Julia Set fractal generator")]
struct Args {
    /// Width of the output image
    #[arg(short, long, default_value_t = 800)]
    width: u32,

    /// Height of the output image
    #[arg(short, long, default_value_t = 800)]
    height: u32,

    /// Maximum number of iterations
    #[arg(short, long, default_value_t = 256)]
    iterations: u32,

    /// Real part of the complex constant c
    #[arg(long, default_value_t = -0.4)]
    c_real: f64,

    /// Imaginary part of the complex constant c
    #[arg(long, default_value_t = 0.6)]
    c_imag: f64,

    /// Minimum x coordinate
    #[arg(long, default_value_t = -1.5)]
    x_min: f64,

    /// Maximum x coordinate
    #[arg(long, default_value_t = 1.5)]
    x_max: f64,

    /// Minimum y coordinate
    #[arg(long, default_value_t = -1.5)]
    y_min: f64,

    /// Maximum y coordinate
    #[arg(long, default_value_t = 1.5)]
    y_max: f64,

    /// Output file path
    #[arg(short, long, default_value = "output/julia_set.png")]
    output: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn calculate_julia_point(x: u32, y: u32, args: &Args, c: Complex<f64>) -> u8 {
    let width_range = args.x_max - args.x_min;
    let height_range = args.y_max - args.y_min;
    
    let cx = x as f64 / args.width as f64 * width_range + args.x_min;
    let cy = y as f64 / args.height as f64 * height_range + args.y_min;
    let mut z = Complex::new(cx, cy);
    let mut iteration = 0;

    while iteration < args.iterations && z.norm() <= 10.0 {
        z = z * z + c;
        iteration += 1;
    }

    ((iteration as f64 / args.iterations as f64) * 255.0) as u8
}

fn main() {
    let args = Args::parse();
    
    if args.verbose {
        println!("Julia Set Generator");
        println!("==================");
        println!("Resolution: {}x{}", args.width, args.height);
        println!("Complex constant c: {} + {}i", args.c_real, args.c_imag);
        println!("Max iterations: {}", args.iterations);
        println!("Bounds: x[{}, {}], y[{}, {}]", args.x_min, args.x_max, args.y_min, args.y_max);
        println!("Output: {}", args.output);
        println!();
    }

    // Create output directory if it doesn't exist
    if let Some(parent) = Path::new(&args.output).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    let c = Complex::new(args.c_real, args.c_imag);
    let mut julia_set = ImageBuffer::new(args.width, args.height);

    let progress_bar = if args.verbose {
        let pb = ProgressBar::new(args.height as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} rows ({eta})")
                .expect("Failed to set progress bar template")
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    // Process each row in parallel
    let pixels: Vec<Vec<Rgb<u8>>> = (0..args.height)
        .into_par_iter()
        .map(|y| {
            let row: Vec<Rgb<u8>> = (0..args.width)
                .map(|x| {
                    let intensity = calculate_julia_point(x, y, &args, c);
                    // Create a more interesting color scheme
                    let r = intensity;
                    let g = (intensity as f64 * 0.6) as u8;
                    let b = (intensity as f64 * 0.8) as u8;
                    Rgb([r, g, b])
                })
                .collect();
            
            if let Some(ref pb) = progress_bar {
                pb.inc(1);
            }
            row
        })
        .collect();

    if let Some(pb) = progress_bar {
        pb.finish_with_message("Calculation complete");
    }

    // Copy pixels to the image buffer
    for (y, row) in pixels.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            julia_set.put_pixel(x as u32, y as u32, pixel);
        }
    }

    // Save the image
    match julia_set.save(&args.output) {
        Ok(_) => {
            if args.verbose {
                println!("✓ Image saved successfully to: {}", args.output);
            } else {
                println!("Image saved to: {}", args.output);
            }
        }
        Err(e) => {
            eprintln!("✗ Error saving image: {}", e);
            std::process::exit(1);
        }
    }
}