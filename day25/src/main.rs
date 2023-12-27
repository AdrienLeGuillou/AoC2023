use std::collections::HashMap;
use petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::connectivity::stoer_wagner_min_cut as swmc;

fn main() {
    // let input = std::fs::read_to_string("example1.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let g = parse_graph(&input);
    // println!("{g:?}");


    // Part 1 ------------------------------------------------------------------
    let Ok(Some((_, group))) = swmc(&g, |_| Ok::<_, ()>(1)) else { panic!(); };
    println!("{group:?}");
    let g1 = group.len();
    let g2 = g.node_count() - g1;

    println!("g1: {g1}, g2: {g2}");
    println!("P1: {}", g1 * g2);

    // Part 2 ------------------------------------------------------------------

    // println!("P2: {answ}");
}

fn parse_line(s: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let (h, ts) = s.split_once(": ").unwrap();
    for t in ts.split(" ") {
        out.push((h.to_owned(), t.to_owned()));
    }
    out
}

fn parse_edgelist(s: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for l in s.lines() {
        out.append(&mut parse_line(l));
    }
    out
}

fn parse_graph(s: &str) -> UnGraph<String, ()> {
    let mut ni: HashMap<String, NodeIndex> = HashMap::new();
    let el = parse_edgelist(s);

    let mut g = UnGraph::new_undirected();

    for e in el {
        if !ni.contains_key(&e.0) {
            let tmp = g.add_node(e.0.clone());
            ni.insert(e.0.clone(), tmp);
        }

        if !ni.contains_key(&e.1) {
            let tmp = g.add_node(e.1.clone());
            ni.insert(e.1.clone(), tmp);
        }

        let e0 = ni.get(e.0.as_str()).unwrap();
        let e1 = ni.get(e.1.as_str()).unwrap();

        g.add_edge(*e0, *e1, ());
    }

    g
}
