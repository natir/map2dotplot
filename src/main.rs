/*
Copyright (c) 2018 Pierre Marijon <pierre.marijon@inria.fr>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

/* extern care */
extern crate bio;
extern crate clap;
extern crate csv;
extern crate serde;
extern crate svg;

#[macro_use]
extern crate serde_derive;

/* local use */
mod io;
mod image;
mod parse;
mod template;

/* standard use */
use std::fs::File;

/* crates use */
use crate::svg::Node;

fn main() {

    let matches = clap::App::new("map2dotplot")
        .version("0.1 Mewtow")
        .author("Pierre Marijon <pierre@marijon.fr>")
        .about("Generate a dotplot from mapping file")
        .arg(clap::Arg::with_name("input")
             .short("i")
             .long("input")
             .required(true)
             .takes_value(true)
             .help("Input file (in paf)")
        )
        .arg(clap::Arg::with_name("output")
             .short("o")
             .long("output")
             .required(true)
             .takes_value(true)
             .help("Output file (in svg)")
        )
        .arg(clap::Arg::with_name("minimum")
             .short("m")
             .long("minimum-map-length")
             .takes_value(true)
             .default_value("0")
             .help("Show only overlap upper than this value")
        )
        .arg(clap::Arg::with_name("maximum")
             .short("M")
             .long("maximum-map-length")
             .takes_value(true)
             .default_value("18446744073709551615")
             .help("Show only overlap lower than this value")
        )
        .arg(clap::Arg::with_name("base_per_pixel")
             .short("b")
             .long("base-per-pixel")
             .takes_value(true)
             .default_value("1")
             .help("A pixel represent many base")
        ).get_matches();

    let input_path = matches.value_of("input").unwrap();
    let input = File::open(input_path).expect(&format!("Can't open input file {}", input_path));

    let output_path = matches.value_of("output").unwrap();
    let output = File::create(output_path).expect(&format!("Can't open output file {}", input_path));

    let min = matches.value_of("minimum").unwrap().parse::<u64>().expect("Can't parse minimum value");
    let max = matches.value_of("maximum").unwrap().parse::<u64>().expect("Can't parse maximum value");
    let base_per_pixel = matches.value_of("base_per_pixel").unwrap().parse::<f64>().expect("Can't parse base per pixel value");
    
    let mut ref2mod = std::collections::HashMap::new();
    let mut tig2mod = std::collections::HashMap::new();
    let (ref_cum_len, tig_cum_len, mappings) = parse::parse(&input, &mut ref2mod, &mut tig2mod, min, max);
    
    let document = image::svg(ref_cum_len, tig_cum_len, ref2mod, tig2mod, mappings, base_per_pixel);

    template::write(document, output);
}
