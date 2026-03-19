use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Team {
    pub(crate) id: i32,
    pub(crate) pot: i32,
    pub(crate) country_code: i32,
    pub(crate) league_ops: [[i32; 2]; 4],
    pub(crate) rating: i32,
}

impl Debug for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Team {{id: {}, pot: {}, cc: {}, ops: [{}, {}, {}, {}, {}, {}, {}, {}] }}",
            self.id,
            self.pot,
            self.country_code,
            self.league_ops[0][0],
            self.league_ops[0][1],
            self.league_ops[1][0],
            self.league_ops[1][1],
            self.league_ops[2][0],
            self.league_ops[2][1],
            self.league_ops[3][0],
            self.league_ops[3][1]
        )
    }
}

pub fn create_team(
    id: i32,
    pot: i32,
    cc: i32,
    pot1_ops: [i32; 2],
    pot2_ops: [i32; 2],
    pot3_ops: [i32; 2],
    pot4_ops: [i32; 2],
    rating: i32,
) -> Team {
    let team = Team {
        id: id,
        pot: pot,
        country_code: cc,
        league_ops: [pot1_ops, pot2_ops, pot3_ops, pot4_ops],
        rating: rating,
    };

    return team;
}
