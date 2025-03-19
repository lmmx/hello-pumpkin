## 10: Reification

Shows how to use reified constraints (turning constraints on/off based on boolean variables).

We have 3 variables: x, y, z each in [0, 10]. We want to enforce: (b1 -> x + y = 10) AND (b2 -> y + z = 15), where b1 and b2 are boolean variables. Additionally, at least one of b1 or b2 must be true.

### Output

```
Reified constraints demonstration:
We have 3 variables: x, y, z each in [0, 10]
We want to enforce: (b1 -> x + y = 10) AND (b2 -> y + z = 15)
Where b1 and b2 are boolean variables
Additionally, at least one of b1 or b2 must be true
Solution 1:
  Variables: x=0, y=10, z=5
  Boolean variables: b1=true, b2=true
  Verification:
  - x + y = 10 (constraint active: true)
    b1 is true, so x + y should be 10: true
  - y + z = 15 (constraint active: true)
    b2 is true, so y + z should be 15: true
  - At least one of b1 or b2 is true: true
Solution 2:
  Variables: x=1, y=9, z=6
  Boolean variables: b1=true, b2=true
  Verification:
  - x + y = 10 (constraint active: true)
    b1 is true, so x + y should be 10: true
  - y + z = 15 (constraint active: true)
    b2 is true, so y + z should be 15: true
  - At least one of b1 or b2 is true: true
Solution 3:
  Variables: x=2, y=8, z=7
  Boolean variables: b1=true, b2=true
  Verification:
  - x + y = 10 (constraint active: true)
    b1 is true, so x + y should be 10: true
  - y + z = 15 (constraint active: true)
    b2 is true, so y + z should be 15: true
  - At least one of b1 or b2 is true: true
Solution 4:
  Variables: x=3, y=7, z=8
  Boolean variables: b1=true, b2=true
  Verification:
  - x + y = 10 (constraint active: true)
    b1 is true, so x + y should be 10: true
  - y + z = 15 (constraint active: true)
    b2 is true, so y + z should be 15: true
  - At least one of b1 or b2 is true: true
Solution 5:
  Variables: x=4, y=6, z=9
  Boolean variables: b1=true, b2=true
  Verification:
  - x + y = 10 (constraint active: true)
    b1 is true, so x + y should be 10: true
  - y + z = 15 (constraint active: true)
    b2 is true, so y + z should be 15: true
  - At least one of b1 or b2 is true: true
Found at least 5 solutions. Stopping the search for brevity.
Found 5 solutions in total.
```