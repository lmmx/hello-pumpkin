use pumpkin_solver::results::{ProblemSolution, SatisfactionResult};
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, Solver};
use pumpkin_solver::results::solution_iterator::IteratedSolution;

fn main() {
    println!("Finding all solutions to x + y = 12");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Create two integer variables x and y, each with a domain [0..24].
    let x = solver.new_bounded_integer(0, 24);
    let y = solver.new_bounded_integer(0, 24);

    // Enforce the constraint: x + y = 12
    _ = solver
        .add_constraint(constraints::equals(vec![x, y], 12))
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
                println!("Solution {}: x={}, y={}", solution_count, x_val, y_val);
            },
            IteratedSolution::Finished => {
                println!("No more solutions exist.");
                break;
            },
            IteratedSolution::Unknown => {
                println!("The solver terminated without finding all solutions.");
                break;
            },
            IteratedSolution::Unsatisfiable => {
                println!("Problem is unsatisfiable.");
                break;
            }
        }
    }
    
    println!("Found {} solutions in total.", solution_count);
}