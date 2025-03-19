use pumpkin_solver::constraints::Constraint;
use pumpkin_solver::results::{ProblemSolution, SatisfactionResult};
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::variables::TransformableVariable;
use pumpkin_solver::{constraints, Solver};

fn main() {
    println!("Demonstrating constraint reification");
    println!("Finding x, y such that:");
    println!("- x, y are in [0, 10]");
    println!("- Either (x >= 5) or (y >= 5) must be true");
    println!("- x + y must be even");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Create two integer variables
    let x = solver.new_bounded_integer(0, 10);
    let y = solver.new_bounded_integer(0, 10);

    // Create boolean literals for our constraints
    let x_geq_5 = solver.new_literal();
    let y_geq_5 = solver.new_literal();
    let sum_is_even = solver.new_literal();

    // Define the reified constraints
    
    // x_geq_5 <=> (x >= 5)
    _ = constraints::less_than_or_equals(vec![solver.make_constant(5)], x)
        .implied_by(&mut solver, x_geq_5, None);
    _ = constraints::less_than_or_equals(vec![x], solver.make_constant(4))
        .implied_by(&mut solver, -x_geq_5, None);

    // y_geq_5 <=> (y >= 5)
    _ = constraints::less_than_or_equals(vec![solver.make_constant(5)], y)
        .implied_by(&mut solver, y_geq_5, None);
    _ = constraints::less_than_or_equals(vec![y], solver.make_constant(4))
        .implied_by(&mut solver, -y_geq_5, None);

    // sum_is_even <=> (x + y) is even
    // We can express this as (x + y) % 2 == 0

    // First, create a variable for (x + y)
    let sum = solver.new_bounded_integer(0, 20);
    _ = solver
        .add_constraint(constraints::equals(vec![x, y], sum))
        .post();

    // Next, create a variable for (x + y) % 2
    let remainder = solver.new_bounded_integer(0, 1);
    _ = solver
        .add_constraint(constraints::modulo(sum, 2, remainder))
        .post();

    // Now define sum_is_even <=> remainder == 0
    _ = constraints::equals(vec![remainder], 0)
        .implied_by(&mut solver, sum_is_even, None);
    _ = constraints::equals(vec![remainder], 1)
        .implied_by(&mut solver, -sum_is_even, None);

    // Add the actual constraints we want to enforce with our reified variables
    // 1. Either x >= 5 or y >= 5
    _ = solver.add_clause([x_geq_5, y_geq_5]);

    // 2. x + y must be even
    _ = solver.add_clause([sum_is_even]);

    // Configure an indefinite termination condition and a default branching strategy
    let mut termination = Indefinite;
    let mut brancher = solver.default_brancher_over_all_propositional_variables();

    // Find a solution
    match solver.satisfy(&mut brancher, &mut termination) {
        SatisfactionResult::Satisfiable(solution) => {
            let x_val = solution.get_integer_value(x);
            let y_val = solution.get_integer_value(y);
            let sum_val = x_val + y_val;
            
            println!("\nFound a solution: x = {}, y = {}", x_val, y_val);
            
            // Verify the constraints
            println!("\nVerifying constraints:");
            println!("- x >= 5: {} ({})", x_val >= 5, if x_val >= 5 { "✓" } else { "✗" });
            println!("- y >= 5: {} ({})", y_val >= 5, if y_val >= 5 { "✓" } else { "✗" });
            println!("- Either x >= 5 or y >= 5: {} ({})", 
                     x_val >= 5 || y_val >= 5, 
                     if x_val >= 5 || y_val >= 5 { "✓" } else { "✗" });
            println!("- x + y = {} is even: {} ({})", 
                     sum_val, 
                     sum_val % 2 == 0, 
                     if sum_val % 2 == 0 { "✓" } else { "✗" });
            
            // Print values of the reified literals
            let x_geq_5_val = solution.get_value(x_geq_5);
            let y_geq_5_val = solution.get_value(y_geq_5);
            let sum_is_even_val = solution.get_value(sum_is_even);
            
            println!("\nReified literals values:");
            println!("- x_geq_5: {}", x_geq_5_val);
            println!("- y_geq_5: {}", y_geq_5_val);
            println!("- sum_is_even: {}", sum_is_even_val);
        }
        SatisfactionResult::Unsatisfiable => {
            println!("No solution exists for the given constraints.");
        }
        SatisfactionResult::Unknown => {
            println!("The solver could not determine satisfiability within the termination condition.");
        }
    }
}