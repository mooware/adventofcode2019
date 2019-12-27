use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

type OrbitMap<'a> = HashMap<&'a str, &'a str>;
type DepthMap<'a> = HashMap<&'a str, usize>;

// have to put this outside of impl OrbitData so that
// orbits and depths can have different mutability
fn min_transfers_impl<'a>(orbits: &OrbitMap<'a>, depths: &mut DepthMap<'a>,
                          a: (&str, usize), b: (&str, usize)) -> usize {
    let (source, source_depth) = a;
    let (target, target_depth) = b;
    if source == target {
        0
    } else if source_depth > target_depth {
        let source = orbits.get(source).unwrap();
        let transfers = min_transfers_impl(orbits, depths, (source, source_depth - 1), (target, target_depth));
        transfers + 1
    } else {
        let target = orbits.get(target).unwrap();
        let transfers = min_transfers_impl(orbits, depths, (source, source_depth), (target, target_depth - 1));
        transfers + 1
    }
}

struct OrbitData<'a> {
    orbits: OrbitMap<'a>,
    depths: DepthMap<'a>
}

impl<'a> OrbitData<'a> {
    fn calculate_depth(&mut self, orbiter: &'a str) -> usize {
        if let Some(&d) = self.depths.get(orbiter) {
            d
        } else if let Some(&inner) = self.orbits.get(orbiter) {
            let depth = self.calculate_depth(inner) + 1;
            self.depths.insert(orbiter, depth); // cache the result
            depth
        } else {
            0
        }
    }

    fn add(&mut self, outer: &'a str, inner: &'a str) {
        self.orbits.insert(outer, inner);
    }

    fn min_transfers(&mut self, a: &str, b: &str) -> usize {
        if let (Some(&source), Some(&target)) = (self.orbits.get(a), self.orbits.get(b)) {
            // because the orbits form a tree, the minimum transfer length should
            // go through the object with the maximum depth which both are orbiting
            let source_depth = self.calculate_depth(source);
            let target_depth = self.calculate_depth(target);
            min_transfers_impl(&self.orbits, &mut self.depths,
                (source, source_depth), (target, target_depth))
        } else {
            0
        }
    }

    fn new() -> Self {
        OrbitData { orbits: HashMap::new(), depths: HashMap::new() }
    }
}

fn main() -> io::Result<()> {
    // reading into a single String allows using &str afterwards
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut data = OrbitData::new();
    for line in input.lines() {
        let mut parts = line.split(')');
        let inner = parts.next().unwrap();
        let outer = parts.next().unwrap();
        data.add(outer, inner);
    }

    let transfers = data.min_transfers("YOU", "SAN");
    println!("{}", transfers);
    Ok(())
}
