use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

type OrbitMap<'a> = HashMap<&'a str, &'a str>;
type DepthMap<'a> = HashMap<&'a str, usize>;

fn calculate_depth<'a>(orbits: &OrbitMap<'a>, depths: &mut DepthMap<'a>, orbiter: &'a str) -> usize {
    if let Some(&d) = depths.get(orbiter) {
        d
    } else if let Some(&inner) = orbits.get(orbiter) {
        let depth = calculate_depth(orbits, depths, inner) + 1;
        depths.insert(orbiter, depth); // cache the result
        depth
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    // reading into a single String allows using &str afterwards
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut orbits = OrbitMap::new();
    for line in input.lines() {
        let mut parts = line.split(')');
        let inner = parts.next().unwrap();
        let outer = parts.next().unwrap();
        orbits.insert(outer, inner);
    }

    let mut depths = DepthMap::new();
    let mut sum = 0;
    for orbiter in orbits.keys() {
        sum += calculate_depth(&orbits, &mut depths, orbiter);
    }

    println!("{}", sum);
    Ok(())
}
