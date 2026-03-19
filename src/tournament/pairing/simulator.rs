use crate::tournament::fixture::Fixture;
use crate::tournament::team::Team;
use rand::RngExt;
use rand_distr::{Distribution, Poisson};

struct FixtureResult {
    fixture: Fixture,
    team1_goals: i32,
    team2_goals: i32,
    winner: i32,
}

fn simulate_fixture(fix: Fixture) -> FixtureResult {
    let base_goals = 1.3;
    let fix_clone = fix.clone();

    let home_advantage = 1.1;

    let (rating_home, rating_away) = if fix.home_team_id == fix.team1.id {
        (fix.team1.rating, fix.team2.rating)
    } else {
        (fix.team2.rating, fix.team1.rating)
    };

    let lam_home = base_goals * (rating_home as f64 / rating_away as f64) * home_advantage;
    let lam_away = base_goals * (rating_away as f64 / rating_home as f64);

    let poihome = Poisson::new(lam_home).unwrap();
    let v_home = poihome.sample(&mut rand::rng());

    let poiaway = Poisson::new(lam_away).unwrap();
    let v_away = poiaway.sample(&mut rand::rng());

    let (team1_goals, team2_goals) = if fix.home_team_id == fix.team1.id {
        (v_home as i32, v_away as i32)
    } else {
        (v_away as i32, v_home as i32)
    };

    println!("{:#?}, {} : {}", fix.clone(), team1_goals, team2_goals);

    if team1_goals > team2_goals {
        return FixtureResult {
            fixture: (fix_clone),
            team1_goals,
            team2_goals,
            winner: (1),
        };
    } else if team1_goals == team2_goals {
        return FixtureResult {
            fixture: (fix_clone),
            team1_goals,
            team2_goals,
            winner: (0),
        };
    } else {
        return FixtureResult {
            fixture: (fix_clone),
            team1_goals,
            team2_goals,
            winner: (2),
        };
    }
}

pub fn simulate_phase(fix_vec: Vec<Fixture>, lt: bool) -> Vec<Team> {
    let mut phase_winners: Vec<Team> = Vec::new();

    if lt {
        let mut counter: usize = 0;

        while counter < fix_vec.len() {
            let result1: FixtureResult = simulate_fixture(fix_vec[counter].clone());
            let result2: FixtureResult = simulate_fixture(fix_vec[counter + 1].clone());

            let prod = result1.winner * result2.winner;
            let sum = result1.winner + result2.winner;

            match prod {
                0 => {
                    if sum == 0 {
                        let rnd = rand::rng().random_range(1..=2);

                        if rnd == 1 {
                            phase_winners.push(fix_vec[counter].team1.clone());
                        } else {
                            phase_winners.push(fix_vec[counter].team2.clone());
                        }
                    } else if sum == 1 {
                        phase_winners.push(fix_vec[counter].team1.clone());
                    } else if sum == 2 {
                        phase_winners.push(fix_vec[counter].team2.clone());
                    }
                }

                2 => {
                    if result1.team1_goals + result2.team1_goals
                        > result1.team2_goals + result2.team2_goals
                    {
                        phase_winners.push(fix_vec[counter].team1.clone());
                    } else {
                        phase_winners.push(fix_vec[counter].team2.clone());
                    }
                }

                1 => phase_winners.push(fix_vec[counter].team1.clone()),
                4 => phase_winners.push(fix_vec[counter].team2.clone()),

                _other => {}
            }

            counter += 2;
        }
    } else {
        for fix in fix_vec {
            let result: FixtureResult = simulate_fixture(fix);

            match result.winner {
                1 => phase_winners.push(result.fixture.team1.clone()),
                2 => phase_winners.push(result.fixture.team2.clone()),

                _other => {
                    let rnd = rand::rng().random_range(1..=2);

                    if rnd == 1 {
                        phase_winners.push(result.fixture.team1.clone());
                    } else {
                        phase_winners.push(result.fixture.team2.clone());
                    }
                }
            }
        }
    }

    return phase_winners;
}
