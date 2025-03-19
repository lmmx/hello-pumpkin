use pumpkin_solver::results::{OptimisationResult, ProblemSolution};
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, Solver};

fn main() {
    println!("Simple machine scheduling with the cumulative constraint");
    println!("Scheduling 4 tasks with different durations and resource requirements");
    println!("Goal: Minimize the makespan (completion time of all tasks)");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Define 4 tasks with: [duration, resource_requirement]
    let tasks = [
        [3, 2], // Task 0: duration=3, resources=2
        [2, 3], // Task 1: duration=2, resources=3
        [4, 1], // Task 2: duration=4, resources=1
        [1, 4], // Task 3: duration=1, resources=4
    ];

    // Maximum resource capacity available at any time
    let capacity = 4;
    
    // Calculate a horizon (upper bound on the makespan)
    let horizon: i32 = tasks.iter().map(|t| t[0]).sum();
    
    // Create start time variables for each task
    let start_times: Vec<_> = tasks
        .iter()
        .map(|task| solver.new_bounded_integer(0, horizon - task[0]))
        .collect();

    // Create durations and resource requirements arrays for the cumulative constraint
    let durations: Vec<i32> = tasks.iter().map(|t| t[0]).collect();
    let resource_reqs: Vec<i32> = tasks.iter().map(|t| t[1]).collect();

    // Post the cumulative constraint
    _ = solver
        .add_constraint(constraints::cumulative(
            start_times.clone(),
            durations,
            resource_reqs,
            capacity,
        ))
        .post();

    // Create a makespan variable representing when all tasks are done
    let makespan = solver.new_bounded_integer(0, horizon);

1 * makespan.scaled(1)))
            .post();
    }

    // Configure an indefinite termination condition and a default branching strategy
    let mut termination = Indefinite;
    let mut brancher = solver.default_brancher_over_all_propositional_variables();

    // Maximize the negative of makespan (equivalent to minimizing makespan)
    match solver.maximise(&mut brancher, &mut termination, makespan.scaled(-1)) {
        OptimisationResult::Optimal(solution) => {
            let makespan_val = solution.get_integer_value(makespan);
            
            println!("\nOptimal solution found with makespan: {}", makespan_val);
            println!("\nTask scheduling details:");
            println!("Task | Start | End | Duration | Resources");
            println!("-----------------------------------------");
            
            // Print details of each task
            for (i, start) in start_times.iter().enumerate() {
                let start_val = solution.get_integer_value(*start);
                let end_val = start_val + tasks[i][0];
                println!(" {}   |   {}   |  {}  |    {}     |    {}    ",
                         i, start_val, end_val, tasks[i][0], tasks[i][1]);
            }
            
            // Print a simple visualization of the schedule
            println!("\nSchedule visualization (time units):");
            println!("0    1    2    3    4    5    6    7    8    9");
            println!("---------------------------------------------- ");
            
            for (i, start) in start_times.iter().enumerate() {
                let start_val = solution.get_integer_value(*start) as usize;
                let end_val = start_val + tasks[i][0] as usize;
                
                // Create a visualization line
                let mut line = " ".repeat(50);
                for t in start_val..end_val {
                    if t < line.len() {
                        line.replace_range(t..t+1, &i.to_string());
                    }
                }
                println!("{}", line);
            }
            
            // Verify that the resource constraint is satisfied
            println!("\nVerifying resource constraints are satisfied at each time point:");
            for t in 0..=makespan_val {
                let mut used_resources = 0;
                for (i, start) in start_times.iter().enumerate() {
                    let start_val = solution.get_integer_value(*start);
                    let end_val = start_val + tasks[i][0];
                    if start_val <= t && t < end_val {
                        used_resources += tasks[i][1];
                    }
                }
                println!("Time {}: {} resources used (capacity: {})", 
                         t, used_resources, capacity);
            }
        }
        OptimisationResult::Satisfiable(solution) => {
            println!("Found a solution but optimality not proven.");
        }
        OptimisationResult::Unsatisfiable => {
            println!("Problem is unsatisfiable.");
        }
        OptimisationResult::Unknown => {
            println!("Could not determine optimality within the termination condition.");
        }
    }
}