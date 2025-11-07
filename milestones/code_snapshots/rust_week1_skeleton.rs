// Week 1: Skeleton
// Minimal CLI skeleton to establish project structure and goals
// See `week1_PROPOSAL.md` for the 1-2 page proposal, wireframes and work plan.

fn main() {
    println!("Rust Task Scheduler — Week 1");
    println!(
        "Goals:\n - Manage scheduled tasks\n - Persist tasks to disk\n - Provide GUI later (Iced)"
    );

    // This skeleton intentionally references the modules that are implemented in
    // subsequent week snapshots. The idea is that each week's file incrementally
    // adds functionality until the final `rust_week7_final.rs` contains the full
    // application.

    println!("Snapshot progression: week2 -> model, week3 -> persistence, week4 -> CLI, week5 -> scheduler core, week6 -> GUI shell, week7 -> final app");

    // Example usage (illustrative only): the final app will call into the
    // data and persistence modules like so (pseudocode):
    //
    // let mut tasks = rust_week3_persistence::load_tasks().unwrap_or_default();
    // tasks.push(rust_week2_model::Task::new("Example", rust_week2_model::Interval::Daily));
    // rust_week3_persistence::save_tasks(&tasks).unwrap();
    //
    // The actual implementations live in `rust_week2_model.rs` and
    // `rust_week3_persistence.rs` snapshots.
}
