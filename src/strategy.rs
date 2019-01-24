use crate::track::Track;

pub trait Strategy {
    //return the index, not the value of the chosen card
    fn rouler(&mut self, cards: &[usize], track: &Track) -> usize;

    //return the index, not the value of the chosen card
    fn sprinter(&mut self, cards: &[usize], track: &Track) -> usize;
    fn strat_name(&self) -> &'static str;
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
