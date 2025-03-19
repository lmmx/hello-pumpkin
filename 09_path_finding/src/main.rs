use pumpkin_solver::results::ProblemSolution;
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, predicate, Solver, variables::IntegerVariable};

fn main() {
    println!("Path finding problem in a small graph:");
    println!("We have 6 nodes (0-5) connected by edges");
    println!("We want to find a path from node 0 to node 5 of length 3 (i.e., 4 nodes)");
    println!("Each node in the path must be connected to the next by an edge");

    // Define our graph: a list of edges (node1, node2)
    let edges = vec![
        (0, 1), (0, 2), 
        (1, 2), (1, 3), 
        (2, 3), (2, 4), 
        (3, 4), (3, 5), 
        (4, 5)
    ];
    
    // Print the graph structure
    println!("\nGraph structure:");
    println!("Edges: {:?}", edges);
    
    // Create a solver with default settings
    let mut solver = Solver::default();

    // Path length (number of nodes - 1 = number of edges in the path)
    let path_length = 3;
    
    // Create variables for each position in the path
    let mut path = Vec::new();
    for i in 0..=path_length {
        path.push(solver.new_bounded_integer(0, 5));
    }
    
    // Start at node 0 and end at node 5
    _ = solver
        .add_constraint(constraints::equals(vec![path[0]], 0))
        .post();
    
    _ = solver
        .add_constraint(constraints::equals(vec![path[path_length]], 5))
        .post();
    
    // All nodes in the path must be different (no cycles)
    _ = solver
        .add_constraint(constraints::all_different(path.clone()))
        .post();
    
    // For each step in the path, the nodes must be connected by an edge
    for i in 0..path_length {
        // Create a disjunction of edge constraints
        let mut edge_constraints = Vec::new();
        
        for (from, to) in &edges {
            // Check if the edge (from, to) matches (path[i], path[i+1])
            let forward_edge = solver.get_literal(predicate!(
                path[i] == *from && path[i+1] == *to
            ));
            
            // Check if the edge (to, from) matches (path[i], path[i+1]) - for undirected graphs
            let backward_edge = solver.get_literal(predicate!(
                path[i] == *to && path[i+1] == *from
            ));
            
            edge_constraints.push(forward_edge);
            edge_constraints.push(backward_edge);
        }
        
        // At least one edge constraint must be satisfied
        _ = solver.add_clause(edge_constraints);
    }
    
    // Configure an indefinite termination condition and a default branching strategy
    let mut termination = Indefinite;
    let mut brancher = solver.default_brancher_over_all_propositional_variables();

    // Solve the problem
    match solver.satisfy(&mut brancher, &mut termination) {
        pumpkin_solver::results::SatisfactionResult::Satisfiable(solution) => {
            println!("\nFound a valid path:");
            
            // Extract the path
            let mut path_values = Vec::new();
            for var in &path {
                path_values.push(solution.get_integer_value(*var));
            }
            
            println!("  Path: {:?}", path_values);
            
            // Verify path constraints
            println!("\nVerification:");
            
            // Start and end nodes
            println!("  Starts at node 0: {}", path_values[0] == 0);
            println!("  Ends at node 5: {}", path_values[path_length] == 5);
            
            // All nodes are different
            let unique_nodes = path_values.iter().collect::<std::collections::HashSet<_>>().len();
            println!("  All nodes are different: {}", unique_nodes == path_values.len());
            
            // Valid edges
            for i in 0..path_length {
                let from = path_values[i];
                let to = path_values[i+1];
                
                let edge_exists = edges.contains(&(from, to)) || edges.contains(&(to, from));
                println!("  Edge ({}, {}) exists: {}", from, to, edge_exists);
            }
        }
        pumpkin_solver::results::SatisfactionResult::Unsatisfiable => {
            println!("\nNo valid path exists for the given constraints.");
        }
        pumpkin_solver::results::SatisfactionResult::Unknown => {
            println!("\nThe solver could not determine if a valid path exists within the termination condition.");
        }
    }
}