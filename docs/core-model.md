
# Core Model

## App

Every application built on Xira ultimately corresponds to a top-level struct `App` as its entry point.
It contains the application logic, the framework runtime, and the data store.

## ID System

Every struct and widget should have an ID.
If a component is not depended on by any other component, its ID can be omitted and is assigned automatically at compile time.

IDs are divided into static IDs and dynamic IDs:
- **Static IDs**: assigned to known instances, embedded at compile time.
- **Dynamic IDs**: assigned to unknown instances (e.g. those generated at runtime), of the form `component name + runtime-assigned number`.
  The numeric part of a dynamic ID is unique within its own component type.

## Data Model

Widgets do not hold data. All data is declared inside `App`, via the `component!` macro.

**Static instances**:
- Their data has a fixed location in `App`, automatically laid out as SoA — data of the same type is stored in the same array, distinguished by index.
- At `App` creation, placeholder values are filled in first (components without a default value must be provided by the developer; the placeholder itself doesn't matter).
- When the component is created, its provided initialization function runs and overwrites the original value.

**Dynamic instances**:
- Not mixed with static instances, but dynamic instances of the same type are stored together, with the index being their numeric ID.

## Dependencies

Dependencies are essentially data relationships.

A dependency relationship is not determined by "who creates whom", but by "who affects whom" — even though in practice the two often coincide.

For example: when a parent component is destroyed, the child is destroyed along with it because the child depends on the parent's layout; without the parent, the child cannot lay itself out.
This kind of "cannot exist independently" relationship is an existence dependency.

The dependency graph must be acyclic. A cycle means the initialization order cannot be determined, and the component logically cannot be created.

Xira's data processing order is:
data change → value dependency propagation → existence check → layout computation → view update

Xira's code analyzer is built on dependency theory:
it extracts dependency edges from the source code, attaches a conditional predicate to each edge, and generates the dependency graph at compile time.
