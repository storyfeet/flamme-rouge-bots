use std::fmt;

#[derive(Copy, Clone)]
pub enum Hill {
    Flat,
    Up,
    Down,
}

#[derive(Copy, Clone)]
pub enum RiderType {
    Sprinter,
    Rouler,
}

#[derive(Copy, Clone)]
pub struct Rider {
    tp: RiderType,
    team: u8,
}

#[derive(Copy, Clone)]
pub struct TRow {
    hill: Hill,
    riders: [Option<Rider>; 2],
}

pub struct Track {
    rows: Vec<TRow>,
}

impl Track {
    pub fn from_rd(v: Vec<(Hill, usize)>) -> Self {
        let mut rows = Vec::new();
        for (hill, n) in v {
            for _ in 0..n {
                rows.push(TRow {
                    hill,
                    riders: [None, None],
                });
            }
        }
        Track { rows }
    }

    pub fn add_riders(&mut self, n: usize) {
        if self.rows.len() < n {
            return;
        }
        for i in 0..n {
            let r = self.rows.get_mut(i).unwrap();
            r.riders[0] = Some(Rider {
                team: i as u8,
                tp: RiderType::Sprinter,
            });
            r.riders[1] = Some(Rider {
                team: i as u8,
                tp: RiderType::Rouler,
            });
        }
    }

    pub fn move_riders(&mut self, v: Vec<(usize, usize)>) {
        //TODO handle hill
        for i in (0..self.rows.len()).rev() {
            for rd in &mut self.rows[i].clone().riders {
                if let Some(rd) = rd.take() {
                    //get distance
                    let dist = match v.get(rd.team as usize) {
                        Some((ds, dr)) => match rd.tp {
                            RiderType::Sprinter => *ds,
                            RiderType::Rouler => *dr,
                        },
                        None => 0,
                    };
                }
            }

            println!("{}", i);
        }
    }

    pub fn print(&self) {
        let v = &self.rows;
        println!("Track");

        for i in 0..(v.len() / 12) + 1 {
            println!("");
            let mut lfstr = "".to_string();
            let mut rtstr = " ".to_string();
            for h in 0..12 {
                if let Some(TRow { hill, riders }) = v.get(i * 12 + h) {
                    let c = match hill {
                        Hill::Flat => '_',
                        Hill::Up => '/',
                        Hill::Down => '\\',
                    };
                    lfstr.push(c);
                    rtstr.push(c);
                    lfstr.push_str(&t_rider_str(riders[1]));
                    rtstr.push_str(&t_rider_str(riders[0]));
                }
            }
            println!("{}", lfstr);
            println!("{}", rtstr);
        }
    }
}

impl fmt::Debug for Rider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.team,
            match self.tp {
                RiderType::Sprinter => 'S',
                _ => 'R',
            }
        )
    }
}
fn t_rider_str(r: Option<Rider>) -> String {
    r.map(|r| format!("{:?}", r)).unwrap_or("--".to_string())
}
