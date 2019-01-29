use crate::track::{Hill, Rider, Track};

pub trait Strategy {
    //return the index, not the value of the chosen card
    fn select(&mut self, r: Rider, cards: &[usize], track: &Track) -> usize;

    fn strat_name(&self) -> &'static str;
}

pub fn prefer(from: &[usize], prefs: &[usize]) -> usize {
    for p in prefs {
        if let Some((n, _)) = from.iter().enumerate().find(|(_, v)| *v == p) {
            return n;
        }
    }

    0
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

pub struct BreakerStrat {
    pub dist: usize,
}

fn breaker_strat(dist: usize, r: Rider, cards: &[usize], track: &Track) -> usize {
    let d = track
        .dist_to_hill(r, Hill::Finish)
        .expect("No Finish at end");
    if d < dist {
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
        breaker_strat(self.dist, r, cards, track)
    }

    //return the index, not the value of the chosen card
    fn strat_name(&self) -> &'static str {
        "Breaker"
    }
}

pub struct HillStrat {}

impl Strategy for HillStrat {
    fn select(&mut self, r: Rider, cards: &[usize], track: &Track) -> usize {
        match track.dist_to_hill(r, Hill::Up) {
            Some(v) if v > 9 => breaker_strat(20, r, cards, track),
            None => breaker_strat(20, r, cards, track),
            Some(v) => {
                let mut best = 0;
                let mut bestpos = 0;
                for (i, c) in cards.iter().enumerate() {
                    if *c < std::cmp::max(v, 5) {
                        if *c > best {
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

pub struct DownStrat {}

impl Strategy for DownStrat {
    fn select(&mut self, r: Rider, cards: &[usize], track: &Track) -> usize {
        match track.rider_on(r){
            (_,Hill::Down)=>return prefer(cards,&[2,3,4]),
            (_,Hill::Up)=>return prefer(cards,&[5,4,3,2,6]),
            _=>{}
        }

        match track.rider_next(r){
            (d,_) if d > 8 => breaker_strat(15,r,cards,track),
            (d,Hill::Down) => prefer(cards,&[d,d+1,d+2]),
            (d,Hill::Up) if d <= 5 => prefer(cards,&[5,4,3,2,6]),
            (d,Hill::Up) => prefer(cards,&[d-1,d-2]),
            (_,Hill::Finish)=> breaker_strat(15,r,cards,track),
            _=>0,
        }
        
    }

    //return the index, not the value of the chosen card
    fn strat_name(&self) -> &'static str {
        "Down"
    }
}
