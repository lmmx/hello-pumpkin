use pumpkin_solver::results::solution_iterator::IteratedSolution;
use pumpkin_solver::results::ProblemSolution;
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, Solver};

fn main() {
    println!("Demonstrating the element constraint:");
    println!("We have an array of 5 variables each in [0, 10]");
    println!("We want to ensure that array[index] = value");
    println!("where index is a variable in [0, 4] and value is a variable in [0, 10]");
    println!("We'll also add a sum constraint: sum(array) = 25");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Create an array of 5 variables each with domain [0, 10]
    let mut array = Vec::new();
    for _ in 0..5 {
        array.push(solver.new_bounded_integer(0, 10));
    }

    // Create variables for the index and value
    let index = solver.new_bounded_integer(0, 4);  // 0-based indexing
    let value = solver.new_bounded_integer(0, 10);

    // Add the element constraint: array[index] = value
    _ = solver
        .add_constraint(constraints::element(array.clone(), index, value))
        .post();

    // Add a constraint that the sum of the array is 25
    _ = solver
        .add_constraint(constraints::equals(array.clone(), 25))
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
                
                // Get values of the array variables
                let mut array_values = Vec::new();
                for var in &array {
                    array_values.push(solution.get_integer_value(*var));
                }
                
                let index_val = solution.get_integer_value(index);
                let value_val = solution.get_integer_value(value);
                
                println!("Solution {}:", solution_count);
                println!("  Array: {:?}", array_values);
                println!("  Index: {}", index_val);
                println!("  Value: {}", value_val);
                
                // Verify constraints
                println!("  Verification:");
                println!("  - array[index] = {} (should be {})", array_values[index_val as usize], value_val);
                println!("  - sum(array) = {} (should be 25)", array_values.iter().sum::<i32>());
                
                // Only show first 5 solutions if there are many
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