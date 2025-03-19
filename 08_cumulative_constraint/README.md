## 8: Cumulative Constraint for Scheduling

Demonstrates the use of the cumulative constraint for a simple scheduling problem. The problem involves scheduling 4 tasks with different durations and resource requirements on a machine with limited capacity. The goal is to minimize the makespan (total completion time).

### Output

```
Simple machine scheduling with the cumulative constraint
Scheduling 4 tasks with different durations and resource requirements
Goal: Minimize the makespan (completion time of all tasks)

Optimal solution found with makespan: 5

Task scheduling details:
Task | Start | End | Duration | Resources
-----------------------------------------
 0   |   0   |  3  |    3     |    2    
 1   |   3   |  5  |    2     |    3    
 2   |   0   |  4  |    4     |    1    
 3   |   0   |  1  |    1     |    4    

Schedule visualization (time units):
0    1    2    3    4    5    6    7    8    9
---------------------------------------------- 
 000                                           
       111                                     
 2222                                          
 3                                             

Verifying resource constraints are satisfied at each time point:
Time 0: 4 resources used (capacity: 4)
Time 1: 3 resources used (capacity: 4)
Time 2: 3 resources used (capacity: 4)
Time 3: 4 resources used (capacity: 4)
Time 4: 3 resources used (capacity: 4)
```