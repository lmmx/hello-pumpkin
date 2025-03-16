use pumpkin_solver::results::solution_iterator::IteratedSolution;
use pumpkin_solver::results::ProblemSolution;
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, Solver};

fn main() {
    println!("Three variable problem: x + y + z = 12, with x in [0,6], y in [6,12], z in [2,8]");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Create three integer variables with different domains
    let x = solver.new_bounded_integer(0, 6); // x in [0, 6]
    let y = solver.new_bounded_integer(6, 12); // y in [6, 12]
    let z = solver.new_bounded_integer(2, 8); // z in [2, 8]

    // Enforce the constraint: x + y + z = 12
    _ = solver
        .add_constraint(constraints::equals(vec![x, y, z], 12))
        .post();

    // Configure an indefinite termination condition and a default branching strategy
    let mut termination = Indefinite;
    let mut brancher = solver.default_brancher_over_all_propositional_variables();

    // Get an iterator over all solutions
    let mut solution_iterator = solver.get_solution_iterator(&mut brancher, &mut termination);

    // Counter for the number of solutions
    let mut solution_count = 0;

    // Iterate over all solutions
    loop {
        match solution_iterator.next_solution() {
            IteratedSolution::Solution(solution) => {
                solution_count += 1;
                let x_val = solution.get_integer_value(x);
                let y_val = solution.get_integer_value(y);
                let z_val = solution.get_integer_value(z);
                println!(
                    "Solution {}: x={}, y={}, z={}",
                    solution_count, x_val, y_val, z_val
                );
                // Verify that our constraint is satisfied
                println!("  x + y + z = {} (should be 12)", x_val + y_val + z_val);
                // Verify that variables are within their domains
                println!("  x in [0,6]: {}", 0 <= x_val && x_val <= 6);
                println!("  y in [6,12]: {}", 6 <= y_val && y_val <= 12);
                println!("  z in [2,8]: {}", 2 <= z_val && z_val <= 8);
            }
            IteratedSolution::Finished => {
                println!("No more solutions exist.");
                break;
            }
            IteratedSolution::Unknown => {
                println!("The solver terminated without finding all solutions.");
                break;
            }
            IteratedSolution::Unsatisfiable => {
                println!("Problem is unsatisfiable.");
                break;
            }
        }
    }

    println!("Found {} solutions in total.", solution_count);
}
