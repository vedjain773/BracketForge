pub mod fixture;
pub mod pairing;
pub mod team;

use crate::tournament::fixture::Fixture;
use crate::tournament::pairing::generator::{gen_pairings, gen_teams, print_fixtures};
use crate::tournament::pairing::simulator::simulate_phase;
use crate::tournament::team::Team;

pub fn simulate_knockout_phase() {
    println!("\nRound of 16:");
    let teams_vec: Vec<Team> = gen_teams(16);
    let fix_vec: Vec<Fixture> = gen_pairings(teams_vec, true);
    print_fixtures(fix_vec.clone());

    println!("\nQuarter finals:");
    let q_teams: Vec<Team> = simulate_phase(fix_vec, true);
    let fix_q_vec: Vec<Fixture> = gen_pairings(q_teams, true);
    print_fixtures(fix_q_vec.clone());

    println!("\nSemi finals:");
    let s_teams: Vec<Team> = simulate_phase(fix_q_vec, true);
    let fix_s_vec: Vec<Fixture> = gen_pairings(s_teams, true);
    print_fixtures(fix_s_vec.clone());

    println!("\nFinals:");
    let f_teams: Vec<Team> = simulate_phase(fix_s_vec, true);
    let fix_f_vec: Vec<Fixture> = gen_pairings(f_teams, false);
    print_fixtures(fix_f_vec.clone());
}
