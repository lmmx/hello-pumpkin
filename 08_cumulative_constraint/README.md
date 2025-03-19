## 8: Cumulative Constraint

Uses the cumulative constraint for a simple scheduling problem.

We have 4 tasks with different durations and resource requirements. Tasks must be scheduled between time 0 and 10. The total resource usage at any time point cannot exceed 5.

Additionally, Task 1 must start after Task 0 ends (precedence constraint).

### Output

```
Scheduling problem with the cumulative constraint:
We have 4 tasks with different durations and resource requirements
Tasks must be scheduled between time 0 and 10
The total resource usage at any time point cannot exceed 5
Found a valid schedule:
  Task 0 starts at time 0, ends at time 2, and uses 1 resources
  Task 1 starts at time 2, ends at time 5, and uses 2 resources
  Task 2 starts at time 5, ends at time 9, and uses 3 resources
  Task 3 starts at time 0, ends at time 2, and uses 2 resources

Schedule visualization (. = free, # = resource unit in use):
Time: 0123456789
Task0: #.........
Task1: ..##......
Task2: .....####..
Task3: ##........

Usage: 30253000

Verification:
  Time 0: Resource usage = 3 (limit = 5)
  Time 1: Resource usage = 3 (limit = 5)
  Time 2: Resource usage = 2 (limit = 5)
  Time 3: Resource usage = 2 (limit = 5)
  Time 4: Resource usage = 2 (limit = 5)
  Time 5: Resource usage = 3 (limit = 5)
  Time 6: Resource usage = 3 (limit = 5)
  Time 7: Resource usage = 3 (limit = 5)
  Time 8: Resource usage = 3 (limit = 5)
  Task 1 starts at 2 which is after Task 0 ends at 2 (should be true): true
```