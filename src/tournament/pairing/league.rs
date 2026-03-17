use crate::tournament::fixture::Fixture;
use crate::tournament::team::Team;
use rand::seq::SliceRandom;

fn team_needs_pot(team: &Team, pot: usize) -> bool {
    return team.league_ops[pot][0] == 0 || team.league_ops[pot][1] == 0;
}

fn team_is_complete(team: &Team) -> bool {
    for pot_ops in team.league_ops {
        if pot_ops[0] == 0 || pot_ops[1] == 0 {
            return false;
        }
    }

    return true;
}

fn add_opponent(team: &mut Team, opponent_id: i32, pot: usize) {
    if team.league_ops[pot][0] == 0 {
        team.league_ops[pot][0] = opponent_id;
    } else {
        team.league_ops[pot][1] = opponent_id;
    }
}

fn already_played(team: &Team, opponent_id: i32) -> bool {
    for pot_ops in team.league_ops {
        if pot_ops[0] == opponent_id || pot_ops[1] == opponent_id {
            return true;
        }
    }

    return false;
}

fn reset_all_teams(teams: &mut Vec<Team>) {
    for team in teams {
        for pot_ops in &mut team.league_ops {
            pot_ops[0] = 0;
            pot_ops[1] = 0;
        }
    }
}
