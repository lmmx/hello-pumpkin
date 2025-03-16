use pumpkin_solver::results::solution_iterator::IteratedSolution;
use pumpkin_solver::results::ProblemSolution;
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, Solver};

fn main() {
    println!("Finding solutions where x, y, z are all different and x + y + z = 4");
    println!("with x, y, z each in [0, 4]");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Create three integer variables, each with a domain [0..4].
    let x = solver.new_bounded_integer(0, 4);
    let y = solver.new_bounded_integer(0, 4);
    let z = solver.new_bounded_integer(0, 4);

    // Enforce the constraint: x + y + z = 4
    _ = solver
        .add_constraint(constraints::equals(vec![x, y, z], 4))
        .post();

    // Add the all_different constraint: x, y, and z must have different values
    _ = solver
        .add_constraint(constraints::all_different(vec![x, y, z]))
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
                // Verify that our constraints are satisfied
                println!("  x + y + z = {} (should be 4)", x_val + y_val + z_val);
                println!(
                    "  All different? {}",
                    x_val != y_val && x_val != z_val && y_val != z_val
                );
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
