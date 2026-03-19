pub mod tournament;

use crate::tournament::simulate_knockout_phase;
use crate::tournament::simulate_league_phase;

fn main() {
    simulate_league_phase();
}
