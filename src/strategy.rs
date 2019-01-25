use crate::track::{Hill, Rider, Track};

pub trait Strategy {
    //return the index, not the value of the chosen card
    fn select(&mut self, r: Rider, cards: &[usize], track: &Track) -> usize;

    fn strat_name(&self) -> &'static str;
}

pub struct RandomStrat {}

impl Strategy for RandomStrat {
    fn select(&mut self, _: Rider, _: &[usize], _: &Track) -> usize {
        0
    }

    fn strat_name(&self) -> &'static str {
        "Random"
    }
}

pub struct HighestStrat {}

impl Strategy for HighestStrat {
    fn select(&mut self, _: Rider, cards: &[usize], _: &Track) -> usize {
        match cards.iter().enumerate().max_by_key(|(_, v)| *v) {
            Some((res, _)) => res,
            _ => 0,
        }
    }

    fn strat_name(&self) -> &'static str {
        "Highest"
    }
}

pub struct BreakerStrat {}

fn breaker_strat(r: Rider, cards: &[usize], track: &Track) -> usize {
    let d = track
        .dist_to_hill(r, Hill::Finish)
        .expect("No Finish at end");
    if d < 20 {
        return match cards.iter().enumerate().max_by_key(|(_, v)| *v) {
            Some((res, _)) => res,
            _ => 0,
        };
    }
    0
}

impl Strategy for BreakerStrat {
    //return the index, not the value of the chosen card
    fn select(&mut self, r: Rider, cards: &[usize], track: &Track) -> usize {
        breaker_strat(r, cards, track)
    }

    //return the index, not the value of the chosen card
    fn strat_name(&self) -> &'static str {
        "Breaker"
    }
}

pub struct HillStrat {}

impl Strategy for HillStrat {
    fn select(&mut self, r: Rider, cards: &[usize], track: &Track) -> usize {
        match track.dist_to_hill(r,Hill::Up){
            Some(v) if v > 9 => breaker_strat(r,cards,track),
            None => breaker_strat(r,cards,track),
            Some(v)=>{
                let mut best = 0;
                let mut bestpos = 0;
                for (i,c) in cards.iter().enumerate(){
                    if *c < std::cmp::max(v,5) {
                        if *c > best{
                            best = *c;
                            bestpos = i;
                        }
                    }
                }
                bestpos
                
            }
        }
    }

    //return the index, not the value of the chosen card
    fn strat_name(&self) -> &'static str {
        "Hill"
    }
}
