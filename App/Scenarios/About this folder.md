# Scenarios

**This is not a module.** It holds the scenarios of `Preparation/Scenarios/`, played as tests through the app's own acts, and nothing in Smoozy depends on it.

- `src/the_desk_machine.rs` is the rented machine at the desk that every scenario test plays against. Its own head says what it is and what this desk cannot do.
- `tests/` holds one file per scenario, named for the scenario.

Why the tests live here rather than beside the desktop's, and what each scenario needs, is in `Audits/Build/Step 12/Step 12 — what each piece is, which files it owns, and in what order.md`, section 1.
