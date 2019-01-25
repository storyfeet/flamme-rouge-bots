mod track;
use crate::track::{Hill, Track};

mod strategy;
use crate::strategy::{HillStrat, HighestStrat, RandomStrat};

mod core;

use std::collections::BTreeMap;

fn main() {
    use self::Hill::*;
    //let mut tk = Track::from_rd(vec![(Flat, 80)]);

    let mut scores = BTreeMap::new();

    for _ in 0..1000 {
        let mut tk = Track::from_rd(vec![(Flat, 5), (Up, 12), (Flat, 20), (Down, 2), (Flat, 9),(Up,4),(Flat,30)]);
        let winners = core::run_race(
            &mut tk,
            &mut vec![
                Box::new(HillStrat {}),
                Box::new(HighestStrat {}),
                Box::new(RandomStrat {}),
                Box::new(RandomStrat {}),
            ],
            0,
            false,
        );
        let winner = winners.last().map(|x|*x).unwrap();
        match scores.get(&winner) {
            Some(n) => scores.insert(winner, n + 1),
            None => scores.insert(winner, 1),
        };
    }

    println!("Winners : {:?}", scores);
}
