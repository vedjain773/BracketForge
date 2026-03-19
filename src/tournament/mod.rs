pub mod fixture;
pub mod pairing;
pub mod team;

use crate::tournament::fixture::Fixture;
use crate::tournament::pairing::generator::{gen_pairings, gen_teams, load_teams};
use crate::tournament::pairing::simulator::simulate_phase;
use crate::tournament::team::Team;

use crate::tournament::pairing::league::league_scheduler;

pub fn simulate_league_phase() {
    let mut teams = load_teams("./data.txt");

    let res = league_scheduler(&mut teams);

    if res {
        println!("Success");
    } else {
        println!("Failed");
    }

    for team in teams {
        println!("{:#?}", team);
    }
}

pub fn simulate_knockout_phase() {
    println!("\nRound of 16:");
    let teams_vec: Vec<Team> = gen_teams(16);
    let fix_vec: Vec<Fixture> = gen_pairings(teams_vec, true);
    let q_teams: Vec<Team> = simulate_phase(fix_vec, true);

    println!("\nQuarter finals:");
    let fix_q_vec: Vec<Fixture> = gen_pairings(q_teams, true);
    let s_teams: Vec<Team> = simulate_phase(fix_q_vec, true);

    println!("\nSemi finals:");
    let fix_s_vec: Vec<Fixture> = gen_pairings(s_teams, true);
    let f_teams: Vec<Team> = simulate_phase(fix_s_vec, true);

    println!("\nFinals:");
    let fix_f_vec: Vec<Fixture> = gen_pairings(f_teams, false);
    let _c_team: Vec<Team> = simulate_phase(fix_f_vec, false);
}
