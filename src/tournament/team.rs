#[derive(Clone)]
pub struct Team {
    pub(crate) id: i32,
    pub(crate) pot: i32,
    pub(crate) country_code: i32,
    pub(crate) league_ops: [[i32; 2]; 4],
    pub(crate) rating: i32,
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
