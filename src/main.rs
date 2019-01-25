mod track;
use crate::track::{Hill,   Track};

mod strategy;
use crate::strategy::{HighestStrat, RandomStrat,BreakerStrat} ;

mod core;





fn main() {
    println!("Hello, world!");

    use self::Hill::*;
    let mut tk = Track::from_rd(vec![(Flat, 5), (Up, 3), (Flat, 20), (Down, 2), (Flat, 9)]);
    //let mut tk = Track::from_rd(vec![(Flat, 80)]);

    let winners = core::run_race(
        &mut tk,
        &mut vec![
            Box::new(RandomStrat {}),
            Box::new(HighestStrat {}),
            Box::new(RandomStrat {}),
            Box::new(BreakerStrat {}),
        ],
    );

    println!("Winners : {:?}", winners);
}
