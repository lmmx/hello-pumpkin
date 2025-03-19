## 10: Constraint Reification

Demonstrates how to use constraint reification, which allows constraints to be turned on or off based on boolean variables. The problem involves finding two variables x and y, where either x or y must be >= 5, and their sum must be even.

### Output

```
Demonstrating constraint reification
Finding x, y such that:
- x, y are in [0, 10]
- Either (x >= 5) or (y >= 5) must be true
- x + y must be even

Found a solution: x = 6, y = 0

Verifying constraints:
- x >= 5: true (✓)
- y >= 5: false (✗)
- Either x >= 5 or y >= 5: true (✓)
- x + y = 6 is even: true (✓)

Reified literals values:
- x_geq_5: true
- y_geq_5: false
- sum_is_even: true
```