use pumpkin_solver::results::{OptimisationResult, ProblemSolution};
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, Solver};

fn main() {
    println!("Finding the solution to x + y = 12 that maximizes x");

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

    // Maximize x
    match solver.maximise(&mut brancher, &mut termination, x) {
        OptimisationResult::Optimal(solution) => {
            let x_val = solution.get_integer_value(x);
            let y_val = solution.get_integer_value(y);
            println!("Optimal solution: x={}, y={}", x_val, y_val);
            println!("Verification: x + y = {}", x_val + y_val);
        }
        OptimisationResult::Satisfiable(solution) => {
            // A valid solution was found, but not guaranteed optimal
            let x_val = solution.get_integer_value(x);
            let y_val = solution.get_integer_value(y);
            println!(
                "Feasible solution (not proven optimal): x={}, y={}",
                x_val, y_val
            );
        }
        OptimisationResult::Unsatisfiable => {
            println!("Problem is unsatisfiable.");
        }
        OptimisationResult::Unknown => {
            println!("The solver could not determine optimality within the termination condition.");
        }
    }
}
