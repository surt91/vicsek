///! two dimensional vicsek model
extern crate rand;

use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io;

use std::process::Command;

mod cell_list;
mod vicsek_model;
mod bird;
mod parse_cl;


fn run(num_birds: u64, num_iterations: u64, neighbors: usize, filename: &str) -> io::Result<()> {
    let mut v = vicsek_model::Vicsek::new(num_birds, neighbors);

    create_dir_all("data")?;
    create_dir_all("img")?;

    let mut file = File::create("plot.gp")?;
    write!(file, "set terminal pngcairo size 1080, 1080\n")?;
    write!(file, "set xr [0:1]\n")?;
    write!(file, "set yr [0:1]\n")?;
    write!(file, "set cbr [-pi:pi]\n")?;
    write!(file, "set palette defined (-pi \"red\", -pi/2 \"blue\", 0 \"green\",  pi/2 \"yellow\", pi \"red\")\n")?;
    write!(file, "set size square\n")?;
    write!(file, "unset tics\n")?;
    write!(file, "unset key\n")?;
    write!(file, "unset colorbox\n")?;
    write!(file, "set style arrow 1 head filled size screen 0.025, 30, 45 ls 1 lc palette\n")?;

    for i in 0..num_iterations {
        let dataname = format!("data/{}_{:04}.dat", filename, i);
        v.save(&dataname)?;

        write!(file, "set output 'img/{}_{:04}.png'\n", filename, i)?;
        write!(file, "p '{}'  u 1:2:($3*0.03):($4*0.03):5 with vectors arrowstyle 1\n", dataname)?;

        v.sweep(5);
    }

    // TODO call the gnuplot script in parallel
    let _ = Command::new("gnuplot")
                    .arg("plot.gp")
                    .output();

    // ffmpeg -f image2 -pattern_type glob -framerate 30 -i "test_*.png" -vcodec libx264 flockingColorNeighbors7.mp4
    let _ = Command::new("ffmpeg")
                    .arg("-f").arg("image2")
                    .arg("-pattern_type").arg("glob")
                    .arg("-framerate").arg("30")
                    .arg("-i").arg(format!("img/{}_*.png", filename))
                    .arg("-vcodec").arg("libx264")
                    .arg(format!("{}.mp4", filename))
                    .output();

    Ok(())
}

fn main() {
    let o = parse_cl::parse_cl();
    // TODO alternatively show on screen, dont save anything
    run(o.num_birds.unwrap_or(500),
        o.num_steps.unwrap_or(300),
        o.num_neighbors.unwrap_or(4),
        &o.filename.unwrap_or("test".to_owned()),
    ).expect("IO error");
}
