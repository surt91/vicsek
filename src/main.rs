///! two dimensional vicsek model
extern crate rand;

use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io;

use std::process::Command;

mod cell_list;
mod vicsek_model;
mod bird;


fn run(num_birds: u64, num_iterations: u64, c_r: f64) -> io::Result<()> {
    let mut v = vicsek_model::Vicsek::new(num_birds, c_r);

    create_dir_all("data")?;
    create_dir_all("img")?;

    let mut file = File::create("plot.gp")?;
    write!(file, "# render with 'ffmpeg -f image2 -pattern_type glob -framerate 30 -i \"test_*.png\" -vcodec libx264 flocking2.mp4'\n")?;
    write!(file, "set terminal pngcairo size 1080, 1080\n")?;
    write!(file, "set xr [0:1]\n")?;
    write!(file, "set yr [0:1]\n")?;
    write!(file, "set cbr [-pi:pi]\n")?;
    write!(file, "set size square\n")?;
    write!(file, "unset tics\n")?;
    write!(file, "unset key\n")?;
    write!(file, "unset colorbox\n")?;
    write!(file, "set style arrow 1 head filled size screen 0.025, 30, 45 ls 1 lc palette\n")?;

    for i in 0..num_iterations {
        let filename = format!("data/test_{:04}.dat", i);
        v.save(&filename)?;

        write!(file, "set output 'img/test_{:04}.png'\n", i)?;
        write!(file, "p '{}'  u 1:2:($3*0.03):($4*0.03):5 with vectors arrowstyle 1\n", filename)?;

        v.sweep(20);
    }

    // TODO call the gnuplot script in parallel
    let _ = Command::new("gnuplot")
                    .arg("plot.gp")
                    .output();

    Ok(())
}

fn main() {
    // TODO CLAP
    run(300, 100, 0.05).expect("IO error");
}
