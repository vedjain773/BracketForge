use crate::tournament::fixture::Fixture;
use crate::tournament::team::Team;
use rand::{Rng, thread_rng};

struct FixtureResult {
    fixture: Fixture,
    winner: i32,
}

fn simulate_fixture(fix: Fixture) -> FixtureResult {
    let guess = thread_rng().gen_range(0..=2);
    let fix_clone = fix.clone();

    if guess == 0 {
        return FixtureResult {
            fixture: (fix_clone),
            winner: (0),
        };
    } else if guess == 1 {
        return FixtureResult {
            fixture: (fix_clone),
            winner: (1),
        };
    } else {
        return FixtureResult {
            fixture: (fix_clone),
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
                        phase_winners.push(fix_vec[counter].team1.clone());
                    } else {
                        phase_winners.push(fix_vec[counter].team2.clone());
                    }
                }

                1 => phase_winners.push(fix_vec[counter].team1.clone()),
                2 => phase_winners.push(fix_vec[counter].team2.clone()),
                4 => phase_winners.push(fix_vec[counter].team2.clone()),

                other => {}
            }

            counter += 2;
        }
    } else {
        for fix in fix_vec {
            let result: FixtureResult = simulate_fixture(fix);

            match result.winner {
                1 => phase_winners.push(result.fixture.team1.clone()),
                2 => phase_winners.push(result.fixture.team2.clone()),

                other => phase_winners.push(result.fixture.team1.clone()),
            }
        }
    }

    return phase_winners;
}
