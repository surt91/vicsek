extern crate clap;

use self::clap::{App, Arg};

#[derive(Debug)]
pub struct Options {
    pub seed: Option<usize>,
    pub filename: Option<String>,
    pub num_birds: Option<u64>,
    pub num_steps: Option<u64>,
    pub num_neighbors: Option<usize>,
    pub gnuplot: bool,
}

pub fn parse_cl() -> Options {
    let matches = App::new(env!("CARGO_PKG_NAME"))
              .version(env!("CARGO_PKG_VERSION"))
              .about(env!("CARGO_PKG_DESCRIPTION"))
              .author(env!("CARGO_PKG_AUTHORS"))
              .arg(Arg::with_name("gnuplot")
                    .short("g")
                    .long("gnuplot")
                    .help("render via gnuplot and convert with ffmpeg")
              )
              .arg(Arg::with_name("seed")
                    .long("seed")
                    .takes_value(true)
                    .help("the seed for the random number generator ")
              )
              .arg(Arg::with_name("filename")
                    .short("f")
                    .long("filename")
                    .takes_value(true)
                    .help("the name of the outputted image")
              )
              .arg(Arg::with_name("num_birds")
                    .short("b")
                    .long("birds")
                    .takes_value(true)
                    .help("number of birds to simulate")
              )
              .arg(Arg::with_name("num_steps")
                    .short("n")
                    .long("steps")
                    .takes_value(true)
                    .help("number of steps to simulate (Monte Carlo time)")
              )
              .arg(Arg::with_name("num_neighbors")
                    .short("m")
                    .long("nieghbors")
                    .takes_value(true)
                    .help("number of neighbors for the birds to orient")
              )
              .get_matches();

    let gnuplot = matches.is_present("gnuplot");
    let filename = matches.value_of("filename")
                          .and_then(|f| Some(f.to_string()))
                          .or_else(|| None);
    let seed = matches.value_of("seed")
                      .and_then(|s| Some(s.parse::<usize>().expect("seed needs to be an integer")))
                      .or_else(|| None);

    let num_birds = matches.value_of("num_birds")
                           .and_then(|s| Some(s.parse::<u64>().expect("seed needs to be an integer")))
                           .or_else(|| None);

    let num_neighbors = matches.value_of("num_neighbors")
                               .and_then(|s| Some(s.parse::<usize>().expect("seed needs to be an integer")))
                               .or_else(|| None);

    let num_steps = matches.value_of("num_steps")
                           .and_then(|s| Some(s.parse::<u64>().expect("seed needs to be an integer")))
                           .or_else(|| None);


    Options {
        seed,
        filename,
        gnuplot,
        num_birds,
        num_neighbors,
        num_steps
    }
}
