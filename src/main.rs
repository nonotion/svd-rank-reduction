extern crate nalgebra as na;
pub use band_filter::*;
use chrono::prelude::*;
use image::{imageops::*, GrayImage, RgbImage, RgbaImage};
use na::{linalg, ArrayStorage, Dynamic, Matrix, VecStorage, U2, U3};
use noise::{utils::*, Fbm, NoiseFn, OpenSimplex, Perlin, Seedable, Worley};

use std::convert::{From, Into};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Config {
    /// Set input image file.
    input: String,

    /// Set maximum mode to rank reduce. Default to 100.
    #[structopt(name = "Maximum Mode", short = "r")]
    rank: Option<usize>,

    /// Set step size. Default to 2.
    #[structopt(name = "Step Size", short = "s")]
    s: Option<usize>,

    /// Set output file name
    #[structopt(name = "File name", short = "o")]
    file_name: Option<String>,
}

fn main() {
    let cfg = Config::from_args();
    let file_name = cfg
        .file_name
        .unwrap_or(Local::now().format("%Y-%m-%d %H_%M_%S").to_string());
    let rank = cfg.rank.unwrap_or(100);
    let s = cfg.s.unwrap_or(2);

    let mut input = image::open(cfg.input.clone()).unwrap();
    let mut input = input.into_luma8();
    let dimensions: (u32, u32) = input.dimensions();
    //dbg!(dimensions);
    let mut input = input.into_raw();
    //dbg!(input.len());
    let mut input: Vec<f64> = input.into_iter().map(|x| x as f64).collect();
    //dbg!(input.len());
    let input_matrix = na::DMatrix::from_vec(
        (dimensions.0) as usize,
        (dimensions.1) as usize,
        input.clone(),
    );
    let na::linalg::SVD {
        u: U,
        v_t: V_T,
        singular_values: sigma,
    } = input_matrix.svd(true, true);
    let sigma = na::DMatrix::from_diagonal(&sigma);
    let (U, V_T) = (U.unwrap(), V_T.unwrap());
    for r in (1..rank).step_by(s).rev() {
        //Xapprox = U[:,:r] @ S[0:r,:r] @ VT[:r,:]
        let x_hat = U.columns(0, r) * sigma.slice((0, 0), (r, r)) * V_T.rows(0, r);
        //dbg!(x_hat.len());
        let output: Vec<u8> = x_hat.into_iter().map(|x| x.abs() as u8).collect();
        //dbg!(output.len());

        let output_img = GrayImage::from_raw(dimensions.0, dimensions.1, output).unwrap();
        output_img.save(&String::from(
            file_name.clone() + " " + &r.to_string() + ".png",
        ));
        println!("Frame {} Complete!", rank - r);
    }
    //ffmpeg -r 30 -f image2 -s dim.0xdim.1 -i "filename %d.png" -vcodec libx264 -crf 17 -pix_fmt yuv420p filename.mp4
}
