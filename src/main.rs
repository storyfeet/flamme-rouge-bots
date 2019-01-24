mod track;
use crate::track::{Hill, Track,Rider,RiderType};

mod strategy;
use crate::strategy::{NonStrategy,HighestStrategy, Strategy};

use card_deck::Deck;

fn rouler_cards() -> Deck<usize> {
    Deck::new(vec![3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7])
}

fn sprinter_cards() -> Deck<usize> {
    Deck::new(vec![2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 9, 9, 9])
}

fn run_race(tk: &mut Track, ss: &mut Vec<Box<Strategy>>)->Vec<Rider> {
    tk.add_riders(ss.len());
    print!("{}[2J", 27 as char);
    tk.print();

    let mut decks = Vec::new();
    for _ in ss.iter() {
        decks.push((sprinter_cards(), rouler_cards()));
    }

    for _ in 0..tk.rows.len() {
        //if race not finished in 100 turns we have a problem
        //first get choices,
        let mut moves = Vec::new();
        for (k, v) in ss.iter_mut().enumerate() {
            let mut sp_d: Vec<usize> = decks[k].0.draw(4).collect();
            let mut rl_d: Vec<usize> = decks[k].1.draw(4).collect();
            let sp_i = v.sprinter(&sp_d, tk);
            let rl_i = v.rouler(&rl_d, tk);

            moves.push((sp_d.remove(sp_i), rl_d.remove(rl_i)));
            //put on discard

            decks[k].0.push_discards(sp_d);
            decks[k].1.push_discards(rl_d);
        }

        //then run choices

        tk.move_riders(&moves);

        std::thread::sleep(std::time::Duration::from_millis(1500));
        print!("{}[2JTrack", 27 as char);
        tk.print();

        tk.slipstream();
        let ex = tk.exhaust();

        for rd in &ex{
            match rd.tp {
                RiderType::Sprinter=>{decks[rd.team].0.put_discard(2)}
                RiderType::Rouler=>{decks[rd.team].1.put_discard(2)}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(1500));
        print!("{}[2JSlip", 27 as char);
        tk.print();
        println!("Exhaust:{:?}",ex);

        let wn = tk.winners();
        if wn.len() > 0{
            return wn;
        }
    }
    return Vec::new();
}

fn main() {
    println!("Hello, world!");

    use self::Hill::*;
    let mut tk = Track::from_rd(vec![(Flat, 5), (Up, 3), (Flat, 20), (Down, 2), (Flat, 9)]);

    let winners = run_race(
        &mut tk,
        &mut vec![
            Box::new(HighestStrategy {}),
            Box::new(NonStrategy {}),
            Box::new(NonStrategy {}),
            Box::new(NonStrategy {}),
        ],
    );

    println!("Winners : {:?}",winners);

}
