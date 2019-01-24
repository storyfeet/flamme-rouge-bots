use crate::track::{Hill, Rider, Track};

pub trait Strategy {
    //return the index, not the value of the chosen card
    fn rouler(&mut self, cards: &[usize], track: &Track) -> usize;

    //return the index, not the value of the chosen card
    fn sprinter(&mut self, cards: &[usize], track: &Track) -> usize;
    fn strat_name(&self) -> &'static str;
    fn set_team(&mut self, _: usize) {}
}

pub struct NonStrategy {}

impl Strategy for NonStrategy {
    fn rouler(&mut self, _: &[usize], _: &Track) -> usize {
        0
    }

    //return the index, not the value of the chosen card
    fn sprinter(&mut self, _: &[usize], _: &Track) -> usize {
        0
    }

    fn strat_name(&self) -> &'static str {
        "Non Strat"
    }
}

pub struct HighestStrategy {}

impl Strategy for HighestStrategy {
    fn rouler(&mut self, cards: &[usize], _: &Track) -> usize {
        match cards.iter().enumerate().max_by_key(|(_, v)| *v) {
            Some((res, _)) => res,
            _ => 0,
        }
    }

    //return the index, not the value of the chosen card
    fn sprinter(&mut self, cards: &[usize], _: &Track) -> usize {
        match cards.iter().enumerate().max_by_key(|(_, v)| *v) {
            Some((res, _)) => res,
            _ => 0,
        }
    }

    fn strat_name(&self) -> &'static str {
        "Non Strat"
    }
}

pub struct BreakerStrategy {
    pub team: usize,
}

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

impl Strategy for BreakerStrategy {
    fn set_team(&mut self, t: usize) {
        self.team = t;
    }
    //return the index, not the value of the chosen card
    fn rouler(&mut self, cards: &[usize], track: &Track) -> usize {
        let r = Rider::rouler(self.team);
        breaker_strat(r, cards, track)
    }

    //return the index, not the value of the chosen card
    fn sprinter(&mut self, cards: &[usize], track: &Track) -> usize {
        let r = Rider::sprinter(self.team);
        breaker_strat(r, cards, track)
    }
    fn strat_name(&self) -> &'static str {
        "Breaker"
    }
}
