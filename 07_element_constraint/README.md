## 7: Element Constraint

Demonstrates the element constraint for array indexing.

We have an array of 5 variables, each in [0, 10]. We want to ensure that array[index] = value, where index is a variable in [0, 4] and value is a variable in [0, 10]. We also add a sum constraint: sum(array) = 25.

### Output

```
Demonstrating the element constraint:
We have an array of 5 variables each in [0, 10]
We want to ensure that array[index] = value
where index is a variable in [0, 4] and value is a variable in [0, 10]
We'll also add a sum constraint: sum(array) = 25
Solution 1:
  Array: [5, 5, 5, 5, 5]
  Index: 0
  Value: 5
  Verification:
  - array[index] = 5 (should be 5)
  - sum(array) = 25 (should be 25)
Solution 2:
  Array: [5, 5, 5, 5, 5]
  Index: 1
  Value: 5
  Verification:
  - array[index] = 5 (should be 5)
  - sum(array) = 25 (should be 25)
Solution 3:
  Array: [5, 5, 5, 5, 5]
  Index: 2
  Value: 5
  Verification:
  - array[index] = 5 (should be 5)
  - sum(array) = 25 (should be 25)
Solution 4:
  Array: [5, 5, 5, 5, 5]
  Index: 3
  Value: 5
  Verification:
  - array[index] = 5 (should be 5)
  - sum(array) = 25 (should be 25)
Solution 5:
  Array: [5, 5, 5, 5, 5]
  Index: 4
  Value: 5
  Verification:
  - array[index] = 5 (should be 5)
  - sum(array) = 25 (should be 25)
Found at least 5 solutions. Stopping the search for brevity.
Found 5 solutions in total.
```