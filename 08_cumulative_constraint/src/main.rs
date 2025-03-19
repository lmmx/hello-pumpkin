use pumpkin_solver::results::ProblemSolution;
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, Solver, variables::IntegerVariable};

fn main() {
    println!("Scheduling problem with the cumulative constraint:");
    println!("We have 4 tasks with different durations and resource requirements");
    println!("Tasks must be scheduled between time 0 and 10");
    println!("The total resource usage at any time point cannot exceed 5");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Create 4 tasks with start times between 0 and 10
    let num_tasks = 4;
    let mut start_times = Vec::new();
    for _ in 0..num_tasks {
        start_times.push(solver.new_bounded_integer(0, 10));
    }

    // Fixed durations for each task
    let durations = vec![2, 3, 4, 2];

    // Fixed resource requirements for each task
    let resources = vec![1, 2, 3, 2];

    // Maximum resource capacity at any time
    let capacity = 5;

    // Add the cumulative constraint
    _ = solver
        .add_constraint(constraints::cumulative(
            start_times.clone(),
            durations.clone(),
            resources.clone(),
            capacity,
        ))
        .post();

    // We require task1 to start after task0 ends
    _ = solver
        .add_constraint(constraints::less_than_or_equals(
            vec![
                start_times[0].scaled(1),
                start_times[1].scaled(-1),
            ],
            -durations[0],
        ))
        .post();

    // Configure an indefinite termination condition and a default branching strategy
    let mut termination = Indefinite;
    let mut brancher = solver.default_brancher_over_all_propositional_variables();

    // Solve the problem
    match solver.satisfy(&mut brancher, &mut termination) {
        pumpkin_solver::results::SatisfactionResult::Satisfiable(solution) => {
            println!("Found a valid schedule:");
            
            // Display the solution
            let mut task_schedules = Vec::new();
            for i in 0..num_tasks {
                let start = solution.get_integer_value(start_times[i]);
                let end = start + durations[i];
                let resource = resources[i];
                
                task_schedules.push((i, start, end, resource));
                
                println!("  Task {} starts at time {}, ends at time {}, and uses {} resources",
                         i, start, end, resource);
            }
            
            // Visualize the schedule with a simple timeline
            println!("\nSchedule visualization (. = free, # = resource unit in use):");
            let max_time = task_schedules.iter().map(|(_, _, end, _)| *end).max().unwrap_or(0);
            
            // Print timeline header
            print!("Time: ");
            for t in 0..=max_time {
                print!("{}", t % 10);
            }
            println!();
            
            // Print task schedules
            for (i, start, end, resource) in &task_schedules {
                print!("Task{}: ", i);
                for t in 0..=max_time {
                    if *start <= t && t < *end {
                        // Task is active at this time
                        print!("{}", "#".repeat(*resource as usize));
                    } else {
                        // Task is not active at this time
                        print!(".");
                    }
                }
                println!();
            }
            
            // Print resource usage timeline
            print!("Usage: ");
            for t in 0..=max_time {
                let usage: i32 = task_schedules.iter()
                    .filter(|(_, start, end, _)| *start <= t && t < *end)
                    .map(|(_, _, _, resource)| *resource)
                    .sum();
                print!("{}", usage);
            }
            println!();
            
            // Verify resource constraints
            println!("\nVerification:");
            for t in 0..=max_time {
                let usage: i32 = task_schedules.iter()
                    .filter(|(_, start, end, _)| *start <= t && t < *end)
                    .map(|(_, _, _, resource)| *resource)
                    .sum();
                println!("  Time {}: Resource usage = {} (limit = {})", t, usage, capacity);
                assert!(usage <= capacity, "Resource constraint violated at time {}", t);
            }
            
            // Verify precedence constraint: task1 starts after task0 ends
            let task0_end = task_schedules[0].1 + durations[0];
            let task1_start = task_schedules[1].1;
            println!("  Task 1 starts at {} which is after Task 0 ends at {} (should be true): {}",
                     task1_start, task0_end, task1_start >= task0_end);
        }
        pumpkin_solver::results::SatisfactionResult::Unsatisfiable => {
            println!("No valid schedule exists for the given constraints.");
        }
        pumpkin_solver::results::SatisfactionResult::Unknown => {
            println!("The solver could not determine if a valid schedule exists within the termination condition.");
        }
    }
}