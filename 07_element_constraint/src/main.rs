use pumpkin_solver::results::{ProblemSolution, SatisfactionResult};
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, Solver};

fn main() {
    println!("Demonstrating the element constraint");
    println!("Finding index and result such that array[index] = result");
    println!("Given array = [5, 8, 3, 7, 2]");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Define our fixed array of values
    let array = [5, 8, 3, 7, 2];
    let n = array.len();

    // Create an index variable (0 to array.len()-1)
    let index_var = solver.new_bounded_integer(0, (n - 1) as i32);
    
    // Create a result variable that will hold the value of array[index]
    let result_var = solver.new_bounded_integer(0, 10);

    // Apply the element constraint: array[index] = result
    _ = solver
        .add_constraint(constraints::element(array.to_vec(), index_var, result_var))
        .post();

    // Add an additional constraint: result must be greater than 5
    // This is expressed as: 5 <= result
    _ = solver
        .add_constraint(constraints::less_than_or_equals(vec![solver.make_constant(5)], result_var))
        .post();

    // Configure an indefinite termination condition and a default branching strategy
    let mut termination = Indefinite;
    let mut brancher = solver.default_brancher_over_all_propositional_variables();

    // Solve the problem
    match solver.satisfy(&mut brancher, &mut termination) {
        SatisfactionResult::Satisfiable(solution) => {
            let index_val = solution.get_integer_value(index_var);
            let result_val = solution.get_integer_value(result_var);
            
            println!("Found a solution:");
            println!("  index = {}", index_val);
            println!("  result = {}", result_val);
            println!("  Verification: array[{}] = {} (should be {})", 
                     index_val, array[index_val as usize], result_val);
            println!("  Constraint: result > 5 => {} > 5: {}", 
                     result_val, result_val > 5);
        }
        SatisfactionResult::Unsatisfiable => {
            println!("No solution exists for the given constraints.");
        }
        SatisfactionResult::Unknown => {
            println!("The solver could not determine satisfiability within the termination condition.");
        }
    }
}