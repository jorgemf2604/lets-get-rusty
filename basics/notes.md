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
```
let spaces = "   ";
let spaces = spaces.len();
```

## Data Types
We can divide them in scalar types and compound types.

### ***<ins>Scalar types</ins>***
They represent a single value (integers, floating-point numbers, Booleans and characters).
### Integer types
- i8
- i16
- i32 (default in integers)
- i64
- i128
- isize 
- u8
- u16
- u32
- u64
- u128
- usize

The isize and usize depend on the computer's architecture (32-bit or 64-bit) and are used when indexing some sort of collection.

#### Integer overflows 
While in debug mode, if integer overflow occurs, the program will panic at runtime. While in release mode, the program won't panic and the value will wrap around. In order to handle the possibility of overflow there are several methods provided by the stdn library (e.g. checked_* methods)

```
// .checked_add() adds 2 numbers, if overflow happens, None is returned
fn main() {
    let x: u8 = 255;
    let y: u8 = 1;
    let z = x.checked_add(y);

    assert_eq!(z, None);
}
```

### Floating-Point Types
- f32 
- f64 (default)

### Numeric operations 
Using the division operator with integers will truncate towards zero to the nearest integer.

### Boolean type
It has 2 possible values: true and false. The Boolean type is specified using bool.

### The Character type (char)
We use single quotes to specify char literals (as opposed to string literals, which use double quotes).

It has a size of 4 bytes (32 bits) and represents a Unicode Scalar Value. 

### ***<ins>Compound types</ins>***
Group of multiple values. Rust has two primitive compound types: tuples and arrays. 

### Tuples
- group a number of values with different types into one compound type
- fixed length (cannot change)
- a tuple without any value has a special name, unit. This value is used to represent an empty value or an empty return. Expressions implicitily return the unit value if they don't return any other value.

```
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  // we can access an elment using dot notation + index
  let first_value = x.0
  // we can destructure the tuple to get the individual values;
  let (x, y, z) = tup;
```
### Arrays
- Every element must have the same type.
- fixed length (cannot change)They are used to allocate data in the stack. If you want something more flexible use a vector.

Initializing arrays:
```
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Another way of initialize an array:
```
let a = [0; 5] 
// the same as let a = [0,0,0,0,0]
```

Accesing elements in an array (bracket notation)

```
let a = [1, 2, 3, 4, 5];
let first = a[0];
```

#### Accessing invalid indexes in an array
The program will panic at runtime, preventing accessing invalid memory (we are looking at you C).

## Functions
- We must declare the types of each parameter

### Statements and Expressions
- Statements are instructions to perform some action and do not return a value 
- Expressions evaluate to a resultant value.

A new scope block created with curly brackets is an expression:
```
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    // the blocks evaluates to 4
}
```

### functions with return values 
- We do not name return values but we must declare their type after an arrow.
- You can return a value explicitly with return or implicitly (the last expression of the body of the function without semicolon). If we use semicolon the expression will turn into a statement and we will get an error (mismatched types: we said we were going to return a specific type and we returned () - the unit type). 
