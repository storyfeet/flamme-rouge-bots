use std::fmt;
use termion::color;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Hill {
    Flat,
    Up,
    Down,
    Finish,
}

#[derive(Copy, Clone,PartialEq)]
pub enum RiderType {
    Sprinter,
    Rouler,
}

#[derive(Copy, Clone,PartialEq)]
pub struct Rider {
    pub tp: RiderType,
    pub team: usize,
}

#[derive(Copy, Clone)]
pub struct TRow {
    hill: Hill,
    riders: [Option<Rider>; 2],
}

pub struct Track {
    pub rows: Vec<TRow>,
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
        for _ in 0..9 {
            rows.push(TRow {
                hill: Hill::Finish,
                riders: [None, None],
            });
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
                team: i,
                tp: RiderType::Sprinter,
            });
            r.riders[1] = Some(Rider {
                team: i,
                tp: RiderType::Rouler,
            });
        }
    }

    pub fn dist_to_hill(&self,r:Rider,htype:Hill)->Option<usize>{
        let mut dist = None;
        for row in &self.rows{
            for n in 0..2{
                if row.riders[n] == Some(r){
                    dist = Some(0);
                }
            }
            if let Some(d) = dist {
                if row.hill == htype {
                    return Some(d);
                }


                dist = Some(d+1);
            }

             
        }
        None
    }

    fn calc_new_pos(&self, row: usize, dist: usize) -> (usize, usize) {
        //down hill
        let dist = match self.rows[row].hill {
            Hill::Down => std::cmp::max(dist, 5),
            _ => dist,
        };
        //up hill
        let mut first_up = None;
        for i in 0..dist {
            let p = i + row;
            if p > self.rows.len() {
                break;
            }
            if let Hill::Up = self.rows[row + i].hill {
                first_up = Some(i);
                break;
            }
        }
        let dist = match first_up {
            Some(a) if a <= 5 => std::cmp::min(5, dist),
            Some(a) => a - 1,
            None => dist,
        };
        //empty space
        for i in 0..dist {
            let r = row + dist - i;
            for c in 0..2 {
                if let None = self.rows[r].riders[c] {
                    return (r, c);
                }
            }
        }
        return (row, 0);
    }

    pub fn slipstream(&mut self) {
        let mut gap = 0;
        let mut back = None;
        for i in 0..self.rows.len() {
            if self.rows[i].hill == Hill::Up {
                back = None;
                continue;
            }
            if let Some(_) = self.rows[i].riders[0] {
                if gap == 1 {
                    if let Some(b) = back {
                        for j in (b..i).rev() {
                            self.rows[j].riders = self.rows[j - 1].riders;
                        }
                        self.rows[b].riders = [None, None];
                    }
                    //slide forward
                }
                if back == None {
                    back = Some(i);
                }
                gap = 0;
                continue;
            }
            gap += 1;
            if gap > 1 {
                back = None
            }
        }
    }

    pub fn move_riders(&mut self, v: &[(usize, usize)]) {
        for i in (0..self.rows.len()).rev() {
            for j in 0..2 {
                if let Some(rd) = self.rows[i].riders[j].clone() {
                    //get distance
                    let dist = match v.get(rd.team as usize) {
                        Some((ds, dr)) => match rd.tp {
                            RiderType::Sprinter => *ds,
                            RiderType::Rouler => *dr,
                        },
                        None => 0,
                    };
                    let (nr, nc) = self.calc_new_pos(i, dist);
                    self.rows[nr].riders[nc] = Some(rd);
                    self.rows[i].riders[j] = None;
                }
            }
        }
    }

    pub fn exhaust(&self) -> Vec<Rider> {
        let mut res = Vec::new();
        let mut last = None;
        for row in &self.rows {
            match row.riders[0] {
                Some(_) => {
                    last = Some(row.riders);
                }
                None => {
                    if let Some(r) = last {
                        if let Some(v) = r[0] {
                            res.push(v);
                        }
                        if let Some(v) = r[1] {
                            res.push(v);
                        }
                    }
                    last = None;
                }
            }
        }
        res
    }

    pub fn winners(&self) -> Vec<Rider> {
        let mut res = Vec::new();
        for r in self.rows.iter().filter(|r| r.hill == Hill::Finish) {
            for i in (0..2).rev() {
                if let Some(rd) = r.riders[i] {
                    res.push(rd);
                }
            }
        }
        res
    }

    pub fn print(&self) {
        let v = &self.rows;

        for i in 0..(v.len() / 16) + 1 {
            println!("");
            let mut lfstr = "".to_string();
            let mut rtstr = " ".to_string();
            for h in 0..16 {
                if let Some(TRow { hill, riders }) = v.get(i * 16 + h) {
                    let c = match hill {
                        Hill::Flat => format!("{}_",color::Bg(color::Reset)),
                        Hill::Up => format!("{}/",color::Bg(color::LightRed)),
                        Hill::Down => format!("{}\\",color::Bg(color::LightBlue)),
                        Hill::Finish => format!("{}^",color::Bg(color::Reset)),
                    };
                    lfstr.push_str(&c);
                    rtstr.push_str(&c);
                    lfstr.push_str(&t_rider_str(riders[1]));
                    rtstr.push_str(&t_rider_str(riders[0]));
                }
            }
            println!("{}", lfstr);
            println!("{}", rtstr);
        }
    }
}

pub fn team_color(v: usize) -> String {
    format!(
        "{}",
        color::Fg(match v {
            0 => color::AnsiValue::rgb(5, 0, 0),
            1 => color::AnsiValue::rgb(0, 5, 0),
            2 => color::AnsiValue::rgb(0, 0, 5),
            3 => color::AnsiValue::rgb(5, 0, 5),
            4 => color::AnsiValue::rgb(0, 5, 5),
            _ => color::AnsiValue::rgb(5, 5, 0),
        })
    )
}

impl Rider {
    pub fn rouler(team:usize)->Rider{
        Rider{team,tp:RiderType::Rouler}
    }
    pub fn sprinter(team:usize)->Rider{
        Rider{team,tp:RiderType::Sprinter}
    }
    pub fn term_color(&self) -> String {
        team_color(self.team)
    }
}

impl fmt::Debug for Rider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.term_color(),
            self.team,
            match self.tp {
                RiderType::Sprinter => 'S',
                _ => 'R',
            },
            color::Fg(color::Reset),
        )
    }
}
fn t_rider_str(r: Option<Rider>) -> String {
    r.map(|r| format!("{:?}", r)).unwrap_or("--".to_string())
}
