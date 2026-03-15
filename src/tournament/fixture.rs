use crate::tournament::team::Team;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Fixture {
    pub(crate) team1: Team,
    pub(crate) team2: Team,
    home_team_id: i32,
    legged_fixture: bool,
    leg: i32,
}

impl Debug for Fixture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fixture {{ {} vs {}, home: {}, leg: {} }}",
            self.team1.id, self.team2.id, self.home_team_id, self.leg
        )
    }
}

pub fn create_fixture(team1: Team, team2: Team, home_id: i32, lf: bool, leg: i32) -> Fixture {
    let fix = Fixture {
        team1: team1,
        team2: team2,
        home_team_id: home_id,
        legged_fixture: lf,
        leg: leg,
    };

    return fix;
}
