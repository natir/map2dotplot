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

/* local use */
use crate::io;

pub fn parse(input: &std::fs::File, ref2mod: &mut std::collections::HashMap<String, u64>, tig2mod: &mut std::collections::HashMap<String, u64>, min: u64, max: u64) -> (u64, u64, Vec<(String, String, u64, u64, u64, u64)>) {
    let mut ref2len = std::collections::HashMap::new();
    let mut tig2len = std::collections::HashMap::new();
    let mut mappings = Vec::new(); //<(String, String, u64, u64, u64, u64)>
    let mut reader = io::paf::Reader::new(input);
    
    for result in reader.records() {
        let record: &dyn io::MappingRecord = &result.expect("trouble durring input parsing");

        let min_length = std::cmp::min(record.end_a() - record.begin_a(), record.end_b() - record.begin_b());
        let max_length = std::cmp::max(record.end_a() - record.begin_a(), record.end_b() - record.begin_b());

        if min_length < min { continue; }
        if max_length > max { continue; }
        
        tig2len.insert(record.read_a(), record.length_a());
        ref2len.insert(record.read_b(), record.length_b());

        if record.strand() == '+' {
            mappings.push((record.read_a(), record.read_b(), record.begin_a(), record.end_a(), record.begin_b(), record.end_b()));
        } else {
            mappings.push((record.read_a(), record.read_b(), record.begin_a(), record.end_a(), record.end_b(), record.begin_b()));
        }
    }
    
    let mut ref_cum_len = 0;
    let mut ref_order = ref2len.keys().collect::<Vec<&String>>();
    ref_order.sort();
    for reference in ref_order {
        ref2mod.insert(reference.to_string(), ref_cum_len);
        ref_cum_len += ref2len.get(reference).unwrap();
    }

    let mut tig2chr: std::collections::HashMap<String, (String, u64, u64)> = std::collections::HashMap::new();
    for (tig, chr, tig_b, tig_e, _, _) in mappings.iter() {
        let len = tig_e - tig_b;

        if tig2chr.contains_key(&tig.to_string()) && tig2chr.get(tig).unwrap().1 < len {
            tig2chr.insert(tig.to_string(), (chr.to_string(), len, *tig_b));
        } else {
            tig2chr.insert(tig.to_string(), (chr.to_string(), len, *tig_b));            
        }
    }

    let mut tig_chr_begin = tig2chr.iter().map(|(x, v)| (x.to_string(), v.0.to_string(), v.2)).collect::<Vec<(String, String, u64)>>();
    tig_chr_begin.sort_by_key(|(_, chr, begin)| (chr.to_string(), *begin));
    let tig_order = tig_chr_begin.iter().map(|(x, y, _)| x);
    
    let mut tig_cum_len = 0;
    for tig in tig_chr_begin.iter().map(|(x, y, _)| x) {
        tig2mod.insert(tig.to_string(), tig_cum_len);
        tig_cum_len += tig2len.get(tig).unwrap();
    }

    return (ref_cum_len, tig_cum_len, mappings);
}
