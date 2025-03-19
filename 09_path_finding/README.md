## 9: Path Finding

Solves a path finding problem in a small graph.

We have 6 nodes (0-5) connected by edges. We want to find a path from node 0 to node 5 of length 3 (i.e., 4 nodes). Each node in the path must be connected to the next by an edge.

### Output

```
Path finding problem in a small graph:
We have 6 nodes (0-5) connected by edges
We want to find a path from node 0 to node 5 of length 3 (i.e., 4 nodes)
Each node in the path must be connected to the next by an edge

Graph structure:
Edges: [(0, 1), (0, 2), (1, 2), (1, 3), (2, 3), (2, 4), (3, 4), (3, 5), (4, 5)]

Found a valid path:
  Path: [0, 1, 3, 5]

Verification:
  Starts at node 0: true
  Ends at node 5: true
  All nodes are different: true
  Edge (0, 1) exists: true
  Edge (1, 3) exists: true
  Edge (3, 5) exists: true
```