use std::usize::{self, MIN};

use crate::tournament::team::Team;
use rand::seq::SliceRandom;

fn team_needs_pot(team: &Team, pot: usize) -> bool {
    return team.league_ops[pot][0] == -1 || team.league_ops[pot][1] == -1;
}

fn is_slot_empty(team: &Team, pot: usize) -> bool {
    return team.league_ops[pot][0] == -1 || team.league_ops[pot][1] == -1;
}

fn team_is_complete(team: &Team) -> bool {
    for pot_ops in team.league_ops {
        if pot_ops[0] == -1 || pot_ops[1] == -1 {
            return false;
        }
    }

    return true;
}

fn add_opponent(team: &mut Team, opponent_id: i32, pot: usize) {
    if team.league_ops[pot][0] == -1 {
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
            pot_ops[0] = -1;
            pot_ops[1] = -1;
        }
    }
}

fn is_valid_pair(a: usize, b: usize, teams: &Vec<Team>) -> bool {
    if a == b {
        return false;
    }

    let team_a = &teams[a];
    let team_b = &teams[b];

    let pot_a = team_a.pot as usize;
    let id_a = team_a.id;

    let pot_b = team_b.pot as usize;
    let id_b = team_b.id;

    if already_played(team_a, id_b) {
        return false;
    } else if already_played(team_b, id_a) {
        return false;
    }

    if !team_needs_pot(team_a, pot_b) || !team_needs_pot(team_b, pot_a) {
        return false;
    }

    if team_a.country_code == team_b.country_code {
        return false;
    }

    if !is_slot_empty(team_a, pot_b) || !is_slot_empty(team_b, pot_a) {
        return false;
    }

    return true;
}

fn get_candidates(team_id: usize, pot: usize, teams: &Vec<Team>) -> Vec<usize> {
    let mut id_vec: Vec<usize> = Vec::new();

    for team in teams {
        if team.pot as usize == pot {
            if is_valid_pair(team_id, team.id as usize, teams) {
                id_vec.push(team.id as usize);
            }
        }
    }

    return id_vec;
}

fn assign_match(a: usize, b: usize, teams: &mut Vec<Team>) {
    let (left, right) = if a > b {
        teams.split_at_mut(a)
    } else {
        teams.split_at_mut(b)
    };

    if a > b {
        add_opponent(&mut left[b], right[0].id, right[0].pot as usize);
        add_opponent(&mut right[0], left[b].id, left[b].pot as usize);
    } else {
        add_opponent(&mut left[a], right[0].id, right[0].pot as usize);
        add_opponent(&mut right[0], left[a].id, left[a].pot as usize);
    }
}

fn select_team(teams: &Vec<Team>) -> usize {
    let mut ref_count = usize::MAX;
    let mut team_id: usize = MIN;

    for team in teams {
        if team_is_complete(team) {
            continue;
        }

        let pot = select_pot(team);
        let count = get_candidates(team.id as usize, pot, teams).len();

        if count == 0 {
            return team.id as usize;
        }

        if count < ref_count {
            ref_count = count;
            team_id = team.id as usize;
        }
    }

    return team_id;
}

fn select_pot(team: &Team) -> usize {
    for pot in 0..4 {
        if team.league_ops[pot][0] == -1 || team.league_ops[pot][1] == -1 {
            return pot as usize;
        }
    }

    return 0 as usize;
}

fn is_schedule_complete(teams: &Vec<Team>) -> bool {
    for team in teams {
        if !team_is_complete(team) {
            return false;
        }
    }

    return true;
}

fn assign_next_match(teams: &mut Vec<Team>) -> bool {
    let team_id = select_team(teams);
    let pot = select_pot(&teams[team_id]);

    println!("Searching candidates for: {team_id}");
    let mut candidates = get_candidates(team_id, pot, teams);

    if candidates.is_empty() {
        return false;
    }

    let mut rng = rand::rng();

    candidates.shuffle(&mut rng);

    let opp_id = candidates[0];

    assign_match(team_id, opp_id, teams);

    return true;
}

pub fn league_scheduler(teams: &mut Vec<Team>) -> bool {
    reset_all_teams(teams);

    while !is_schedule_complete(teams) {
        let res = assign_next_match(teams);

        if !res {
            return false;
        }
    }

    return true;
}
