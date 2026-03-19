# UEFA Champions League Scheduler (Rust)

## Overview

This project is a constraint-based implementation of the UEFA Champions League league-phase draw, written in Rust. The primary goal is to model and generate valid opponent assignments under a set of non-trivial constraints inspired by the new competition format.

The project is intentionally designed as a learning exercise to explore Rust through a moderately complex, systems-style problem involving graph construction, constraint satisfaction, and algorithmic design.

---

## Current Features

### Opponent Generation

* Each team is assigned exactly 8 opponents
* Opponents are selected under the following constraints:

  * Exactly 2 opponents from each pot
  * No duplicate opponents
  * No matches between teams from the same country
  * Symmetric assignments (if A plays B, then B plays A)

### Heuristic-Based Selection

* Opponents are selected using a constraint-aware heuristic:
  * Prioritizes teams with fewer valid candidate options
* Includes retry-on-failure to handle unsatisfiable intermediate states

## Knockout Phase Simulator

* Bracket generation for elimination rounds
* Fixture simulation, including two-legged ties
* Result simulation incorporating:
  * Home and away advantage
  * Team ratings

---

## Project Structure

The codebase is organized into modular components:

* **pairing/generator**: Loads / Generates team data
* **pairing/league**: Contains all functions for league phase opponent assignment
* **pairing/simulator**: Contains the logic for simulating the results of fixtures 
* **team**: Defines the Team struct
* **fixture**: Defines the fixture struct

---

## Future Work

### Fixture Generation

* Convert opponent pairs into actual matches
* Assign home and away teams
* Ensure each pairing appears exactly once
