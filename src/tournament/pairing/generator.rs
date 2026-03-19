use crate::tournament::fixture::{Fixture, create_fixture};
use crate::tournament::team::{Team, create_team};
use rand::seq::SliceRandom;
use std::fs;

pub fn load_teams(file_path: &str) -> Vec<Team> {
    let mut teams_vec: Vec<Team> = Vec::new();

    let contents = fs::read_to_string(file_path).expect("Could not read file");

    for line in contents.lines() {
        let columns: Vec<&str> = line.split(',').collect();

        let team_to_push = create_team(
            String::from(columns[0]),
            columns[1].parse().expect("Not a valid id"),
            columns[2].parse().expect("Not a valid pot"),
            columns[3].parse().expect("Not a valid country code"),
            [-1, -1],
            [-1, -1],
            [-1, -1],
            [-1, -1],
            columns[4].parse().expect("Not a valid rating"),
        );

        teams_vec.push(team_to_push);
    }

    return teams_vec;
}

pub fn gen_teams(no_of_teams: usize) -> Vec<Team> {
    let mut teams_vec: Vec<Team> = Vec::new();

    let mut counter: usize = 0;
    while counter != no_of_teams {
        let i = match i32::try_from(counter + 1) {
            Ok(v) => v,
            Err(_) => 0,
        };

        let rating = match i32::try_from(100 - counter) {
            Ok(v) => v,
            Err(_) => 0,
        };

        let team_topush = create_team(
            String::from("Team"),
            i,
            0,
            0,
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            rating,
        );

        teams_vec.push(team_topush);

        counter += 1;
    }

    return teams_vec;
}

pub fn gen_pairings(teams_vector: Vec<Team>, lf: bool) -> Vec<Fixture> {
    let mut teams_vec = teams_vector;
    let mut fix_vec: Vec<Fixture> = Vec::new();

    let mut rng = rand::rng();

    teams_vec.shuffle(&mut rng);

    let mut counter: usize = 0;

    while counter < teams_vec.len() / 2 {
        let fix = create_fixture(
            teams_vec[counter].clone(),
            teams_vec[teams_vec.len() - counter - 1].clone(),
            teams_vec[counter].id,
            lf,
            1,
        );

        fix_vec.push(fix);

        if lf {
            let fix_2 = create_fixture(
                teams_vec[counter].clone(),
                teams_vec[teams_vec.len() - counter - 1].clone(),
                teams_vec[teams_vec.len() - counter - 1].id,
                lf,
                2,
            );

            fix_vec.push(fix_2);
        }

        counter += 1;
    }

    return fix_vec;
}

pub fn print_fixtures(fix_vec: Vec<Fixture>) {
    for item in fix_vec {
        println!("{:#?}", item);
    }
}
