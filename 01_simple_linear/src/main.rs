use pumpkin_solver::results::{ProblemSolution, SatisfactionResult};
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, Solver};

fn main() {
    println!("Hello, Pumpkin solver!");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Create two integer variables x and y, each with a domain [0..20].
    let x = solver.new_bounded_integer(0, 24);
    let y = solver.new_bounded_integer(0, 24);

    // Enforce the constraint: x + y = 12
    _ = solver
        .add_constraint(constraints::equals(vec![x, y], 12))
        .post();

    // Configure an indefinite termination condition and a default branching strategy
    let mut termination = Indefinite;
    let mut brancher = solver.default_brancher_over_all_propositional_variables();

    // Attempt to find a single satisfiable solution
    match solver.satisfy(&mut brancher, &mut termination) {
        SatisfactionResult::Satisfiable(solution) => {
            let x_val = solution.get_integer_value(x);
            let y_val = solution.get_integer_value(y);
            println!("Solution found: x={}, y={}", x_val, y_val);
        }
        SatisfactionResult::Unsatisfiable => {
            println!("No solution exists for the given constraints.");
        }
        SatisfactionResult::Unknown => {
            println!(
                "The solver could not determine satisfiability within the termination condition."
            );
        }
    }
}
