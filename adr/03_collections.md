# Collections

## Status

Accepted

## Context

When utilizing the library with a library defined storage schema becomes cumbersome when writing function definitions that pass the storage schema as an argument. This is also a very common occurence so is a giant pain point when interfacing with the library API. An example of the old `pub fn example(grid: HexGrid<i32, (), ()>) {}`. Another factor is what about application code that doesn't want vertex and/or edge support, it ends up being bloat.

## Decision

Removing the library authored storage schema in favor of traits that the library requires of a storage schema.

## Consequences

### Ease
- This allows the application code to look and feel nicer while hiding many of the ugliness inside the domain of the library.
- Modularity and customizable data structures in application code.
### Pain
- There is an extra step in application code to define a custom storage schema and implement the traits neccesary.
- Library code can end up being more complex when interfacing with this collection.
