use pumpkin_solver::results::solution_iterator::IteratedSolution;
use pumpkin_solver::results::ProblemSolution;
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, predicate, Solver, variables::IntegerVariable};

fn main() {
    println!("Reified constraints demonstration:");
    println!("We have 3 variables: x, y, z each in [0, 10]");
    println!("We want to enforce: (b1 -> x + y = 10) AND (b2 -> y + z = 15)");
    println!("Where b1 and b2 are boolean variables");
    println!("Additionally, at least one of b1 or b2 must be true");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Create three integer variables x, y, z each with domain [0, 10]
    let x = solver.new_bounded_integer(0, 10);
    let y = solver.new_bounded_integer(0, 10);
    let z = solver.new_bounded_integer(0, 10);

    // Create two boolean literals representing the reification variables
    let b1 = solver.new_boolean_variable();
    let b2 = solver.new_boolean_variable();
    
    let b1_true = solver.get_literal_from_boolean_variable(b1);
    let b2_true = solver.get_literal_from_boolean_variable(b2);
    
    // Reified constraint: b1 -> x + y = 10
    // This means: IF b1 is true, THEN x + y must equal 10
    // We do this by creating the literal for (x + y = 10), then adding the implication
    let xy_equals_10 = solver.get_literal(predicate!(x + y == 10));
    _ = solver.add_clause(vec![!b1_true, xy_equals_10]);
    
    // Reified constraint: b2 -> y + z = 15
    // This means: IF b2 is true, THEN y + z must equal 15
    let yz_equals_15 = solver.get_literal(predicate!(y + z == 15));
    _ = solver.add_clause(vec![!b2_true, yz_equals_15]);
    
    // At least one of b1 or b2 must be true
    _ = solver.add_clause(vec![b1_true, b2_true]);

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
                
                // Get values
                let x_val = solution.get_integer_value(x);
                let y_val = solution.get_integer_value(y);
                let z_val = solution.get_integer_value(z);
                
                // Get boolean values
                let b1_val = solution.is_literal_true(b1_true);
                let b2_val = solution.is_literal_true(b2_true);
                
                println!("Solution {}:", solution_count);
                println!("  Variables: x={}, y={}, z={}", x_val, y_val, z_val);
                println!("  Boolean variables: b1={}, b2={}", b1_val, b2_val);
                
                // Verify constraints
                println!("  Verification:");
                println!("  - x + y = {} (constraint active: {})", x_val + y_val, b1_val);
                if b1_val {
                    println!("    b1 is true, so x + y should be 10: {}", x_val + y_val == 10);
                }
                
                println!("  - y + z = {} (constraint active: {})", y_val + z_val, b2_val);
                if b2_val {
                    println!("    b2 is true, so y + z should be 15: {}", y_val + z_val == 15);
                }
                
                println!("  - At least one of b1 or b2 is true: {}", b1_val || b2_val);
                
                // Only show first 5 solutions for brevity
                if solution_count >= 5 {
                    println!("Found at least 5 solutions. Stopping the search for brevity.");
                    break;
                }
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