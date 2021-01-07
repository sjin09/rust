## Input to functions and methods

## Minimize assumptions about parameters

### Minimizing  assumptions through generics:

Prefer

```
fn foo<T: Iterator<int>>(c: T) { ... }
```

over any of

```
fn foo(c: &[int]) { ... }
fn foo(c: &Vec<int>) { ... }
fn foo(c: &SomeOtherCollection<int>) { ... }
```

if the function only needs to iterate over the data.

### Minimizing owernship assumptions:

Prefer either of

```
fn foo(b: &Bar) { ... }
fn foo(b: &mut Bar) { ... }
```

over 


```
fn foo(b: Bar) { ... }
```

That is, prefer borrowing arguments rather than transferring ownership, unless ownership is actually needed.

### Prefer compound return types to out-parameters

Prefer

```
fn foo() -> (Bar, Bar)
```

over

```
fn foo(output: &mut Bar) -> Bar
```

Compound return types like tuples and structs are efficiently compiled and do not require heap allocation. If a function needs to return multiple values, it should do so via one of these types.



