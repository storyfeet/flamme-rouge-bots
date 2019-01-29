use card_deck::Deck;

use crate::strategy::Strategy;
use crate::track::{Rider, RiderType, Track};

pub fn rouler_cards() -> Deck<usize> {
    Deck::new(vec![3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7])
}

pub fn sprinter_cards() -> Deck<usize> {
    Deck::new(vec![2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 9, 9, 9])
}

pub fn run_race(
    tk: &mut Track,
    ss: &mut Vec<Box<Strategy>>,
    sleep: u64,
    print: bool,
) -> Vec<Rider> {
    tk.add_riders(ss.len());
    if print {
        print!("{}[2J{}Start", 27 as char, termion::cursor::Goto(1, 1));
        tk.print();
    }

    let mut decks = Vec::new();
    for _ in ss.iter() {
        decks.push((sprinter_cards(), rouler_cards()));
    }

    //if race not finished in less turns than the length of the track
    //there is a problem
    for _ in 0..tk.rows.len() {
        //first get choices,
        let mut moves = Vec::new();
        for (k, v) in ss.iter_mut().enumerate() {
            let mut sp_d: Vec<usize> = decks[k].0.draw(4).collect();
            let mut rl_d: Vec<usize> = decks[k].1.draw(4).collect();

            if sp_d.len() == 0 {
                sp_d.push(2);
            }
            if rl_d.len() == 0 {
                rl_d.push(2);
            }

            let sp_i = v.select(Rider::sprinter(k), &sp_d, tk);
            let rl_i = v.select(Rider::rouler(k), &rl_d, tk);

            moves.push((sp_d.remove(sp_i), rl_d.remove(rl_i)));
            //put on discard

            decks[k].0.push_discards(sp_d);
            decks[k].1.push_discards(rl_d);
        }

        //then run choices

        tk.move_riders(&moves);

        std::thread::sleep(std::time::Duration::from_millis(sleep));
        if print {
            print!("{}[2J{}Move", 27 as char, termion::cursor::Goto(1, 1));
            tk.print();
            println!("{:?}", moves);
            for s in ss.iter() {
                println!(",{}", s.strat_name());
            }
        }

        tk.slipstream();
        let ex = tk.exhaust();

        for rd in &ex {
            match rd.tp {
                RiderType::Sprinter => decks[rd.team].0.put_discard(2),
                RiderType::Rouler => decks[rd.team].1.put_discard(2),
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(sleep));
        if print {
            print!("{}[2J{}Slip", 27 as char, termion::cursor::Goto(1, 1));
            tk.print();
            println!("{:?}", moves);
            for s in ss.iter() {
                println!(",{}", s.strat_name());
            }
            println!("Exhaust:{:?}", ex);
        }

        let wn = tk.winners();
        if wn.len() > 0 {
            return wn;
        }
    }
    return Vec::new();
}
