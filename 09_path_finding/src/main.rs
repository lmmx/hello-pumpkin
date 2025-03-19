use pumpkin_solver::predicates::IntegerPredicate;
use pumpkin_solver::results::{ProblemSolution, SatisfactionResult};
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, predicate, Solver};

fn main() {
    println!("Simple path finding in a graph");
    println!("Finding a path from node 0 to node 4 with length <= 3");

    // Create a solver with default settings
    let mut solver = Solver::default();

    // Number of nodes in our graph
    let n_nodes = 5;
    
    // Define edges in our graph: (from, to)
    let edges = [
        (0, 1), (0, 2),  // Edges from node 0
        (1, 2), (1, 3),  // Edges from node 1
        (2, 3), (2, 4),  // Edges from node 2
        (3, 4),          // Edge from node 3
    ];
    
    // Create variables representing the position of each node in the path
    // -1 means the node is not in the path
    // A value 0..n_nodes-1 indicates the position in the path
    let position = (0..n_nodes)
        .map(|_| solver.new_bounded_integer(-1, (n_nodes - 1) as i32))
        .collect::<Vec<_>>();
    
    // Source node (node 0) must be at position 0
    _ = solver
        .add_constraint(constraints::equals(vec![position[0]], 0))
        .post();

    // Target node (node 4) must be in the path (position >= 0)
    let target_in_path = solver.get_literal(predicate!(position[4] >= 0));
    _ = solver.add_clause([target_in_path]);
    
    // Each position in the path can be occupied by at most one node
    for pos in 0..(n_nodes-1) {
        for i in 0..n_nodes {
            for j in (i+1)..n_nodes {
                let i_at_pos = solver.get_literal(predicate!(position[i] == pos));
                let j_at_pos = solver.get_literal(predicate!(position[j] == pos));
                _ = solver.add_clause([-i_at_pos, -j_at_pos]);
            }
        }
    }
    
    // Nodes at consecutive positions in the path must be connected by an edge
    for i in 0..n_nodes {
        for pos in 0..(n_nodes-2) {
            let i_at_pos = solver.get_literal(predicate!(position[i] == pos));
            
            // If node i is at position pos, then some node j connected to i must be at position pos+1
            let mut clause = vec![-i_at_pos];
            
            for &(from, to) in &edges {
                if from == i {
                    let j_at_next_pos = solver.get_literal(predicate!(position[to] == pos + 1));
                    clause.push(j_at_next_pos);
                } else if to == i {
                    // Consider edges in both directions
                    let j_at_next_pos = solver.get_literal(predicate!(position[from] == pos + 1));
                    clause.push(j_at_next_pos);
                }
            }
            
            _ = solver.add_clause(clause);
        }
    }
    
    // Configure an indefinite termination condition and a default branching strategy
    let mut termination = Indefinite;
    let mut brancher = solver.default_brancher_over_all_propositional_variables();

    // Find a solution
    match solver.satisfy(&mut brancher, &mut termination) {
        SatisfactionResult::Satisfiable(solution) => {
            // Extract the path
            let mut path = vec![];
            let mut max_pos = -1;
            
            for i in 0..n_nodes {
                let pos = solution.get_integer_value(position[i]);
                if pos >= 0 {
                    path.push((pos, i));
                    max_pos = max_pos.max(pos);
                }
            }
            
            // Sort by position
            path.sort_by_key(|&(pos, _)| pos);
            
            println!("\nFound a path from node 0 to node 4:");
            
            let path_nodes: Vec<_> = path.iter().map(|&(_, node)| node).collect();
            println!("Path: {:?}", path_nodes);
            println!("Path length: {}", max_pos);
            
            // Verify that all consecutive nodes in the path are connected by an edge
            println!("\nVerifying that all consecutive nodes in the path are connected:");
            for i in 0..path.len()-1 {
                let (_, node1) = path[i];
                let (_, node2) = path[i+1];
                
                let is_connected = edges.iter().any(|&(from, to)| 
                    (from == node1 && to == node2) || (from == node2 && to == node1)
                );
                
                println!("Edge ({}, {}) exists: {}", node1, node2, is_connected);
                
                if !is_connected {
                    println!("ERROR: The path is invalid!");
                }
            }
        }
        SatisfactionResult::Unsatisfiable => {
            println!("No path exists with the given constraints.");
        }
        SatisfactionResult::Unknown => {
            println!("The solver could not determine satisfiability within the termination condition.");
        }
    }
}