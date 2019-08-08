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

/* crates use */
use crate::svg::Node;

pub fn svg(ref_cum_len: u64, tig_cum_len: u64, ref2mod: std::collections::HashMap<String, u64>, tig2mod: std::collections::HashMap<String, u64>, mappings: Vec<(String, String, u64, u64, u64, u64)>, base_per_pixel: f64) -> String {

    let mut document = svg::Document::new();
    document = document.set("viewBox", (0, 0, ref_cum_len as f64 / base_per_pixel, tig_cum_len as f64 / base_per_pixel));
    document.assign("easypz", "change_me");
    document.assign("width", "100%");
    let easypz_param = format!("'{{\"applyTransformTo\": \"svg > *\", \"options\": {{\"minScale\": 0.1, \"maxScale\": 200, \"bounds\": {{ \"top\": -{}, \"right\": {}, \"bottom\": {}, \"left\": -{} }}}} }}'", ref_cum_len as f64, tig_cum_len as f64 / base_per_pixel, ref_cum_len as f64, tig_cum_len as f64 / base_per_pixel);

    let mut ref_pos: Vec<&u64> = ref2mod.values().collect();
    ref_pos.sort();
    for pos in ref_pos {
        let mut line = svg::node::element::Line::new();
        line.assign("x1", *pos as f64 / base_per_pixel);
        line.assign("x2", *pos as f64 / base_per_pixel);
        line.assign("y1", 0);
        line.assign("y2", ref_cum_len as f64 / base_per_pixel);
        line.assign("stroke", "black");
        line.assign("stroke-dasharray", "5, 10");
        line.assign("stroke-width", 0.4);
        
        document = document.add(line);
    }

    let mut tig_pos: Vec<&u64> = tig2mod.values().collect();
    tig_pos.sort();
    for pos in tig_pos {
        let mut line = svg::node::element::Line::new();
        line.assign("y1", *pos as f64 / base_per_pixel);
        line.assign("y2", *pos as f64 / base_per_pixel);
        line.assign("x1", 0);
        line.assign("x2", tig_cum_len as f64 / base_per_pixel);
        line.assign("stroke", "black");
        line.assign("stroke-dasharray", "5, 10");
        line.assign("stroke-width", 0.4);
        
        document = document.add(line);
    }
    
    for (tig, chr, tig_begin, tig_end, chr_begin, chr_end) in mappings {
        let chrdiff = ref2mod.get(&chr).unwrap();
        let tigdiff = tig2mod.get(&tig).unwrap();
        
        let mut line = svg::node::element::Line::new();
        let title = format!("{} {}-{}, {} {}-{}", tig, tig_begin, tig_end, chr, chr_begin, chr_end);
        
        line.assign("x1", (chr_begin + chrdiff) as f64 / base_per_pixel);
        line.assign("x2", (chr_end + chrdiff) as f64 / base_per_pixel);
        line.assign("y1", (tig_begin + tigdiff) as f64 / base_per_pixel);
        line.assign("y2", (tig_end + tigdiff) as f64 / base_per_pixel);
        line.assign("stroke", "red");
        line.assign("stroke-width", 1);
        line = line.add(svg::node::element::Title::new().add(svg::node::Text::new(title)));

        document = document.add(line);
    }
    let svg = format!("{}", document).replace("\"change_me\"", &easypz_param);

    return svg;
}
