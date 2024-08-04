# Some fundamentals 
## Constants
- They are not inmutable by default (like variables), they are always inmutable.
- They can be declared in any scope (including the global scope).
- Constants can only be set to a constant expression, not the result of a value that could only be computed at runtime. 

```
  const LIVES: u32 = 5;
```
## Shadowing
If we "redeclare" a variable with let, we shadow it.
With shadowing we can perform a few transformations on a value (we can even change the type of the value) but have the variable be inmutable after those transformations. 

### shadowing is different from reassigning
### shadowing is different from changing a mutable variable