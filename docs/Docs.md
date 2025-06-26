# Functions
Functions have a simple and readable syntax:
```duk
fun <name>(<args>): <return_type> {
    <code>
}
```
the return type can be ommitted, defaulting to `Foundation.Primitives.Unit`. The arrow syntax is also supported:
```duk
fun <name>(<args>): <return_type>
    => <expr>
```
Here, `<expr>` automatically gets returned.

# Variables
Variables can be either mutable or immutable. A mutable variable is declared with the `var` keyword, like so:
```duk
var <name>: <type> = <initial_value>
```
and immutable variables in the same way, using the `let` keyword:
```duk
let <name>: <type> = <initial_value>
```
Mutable variables can ommit the type, so the compiler infers it from the initial value. An initial value is required.

# Conditions
For conditions, we use the `if` keyword, optionally followed by `else` or `else if` clauses.
```duk
if <condition> {
    <code>
} else if <condition> {
    <code>
} else <condition> {
    <code>
}
```

# For loop
Can be used as a traditional foreach loop:
```duk
for <iterator> in <iterable> {
    <code>
}
```
or as a while loop:
```duk
for <condition> {
    <code>
}
```

# Attributes
There is currently no way to create custom attributes.
The attribute usage syntax is:
```duk
@<attribute_name>
```
Attributes are metadata added to objects to tell the compiler how the objects should be compiled, or how it should use them. Currently, there are these attributes:
## Function attributes
- `@start`
  Marks the program's entry point

## Method attributes
- `@drop`
  Tells the compiler to call the method once the object instance is dropped.
- `@add`
  Marks the method as the `+` operator overload.
- `@sub`
  Marks the method as the `-` operator overload.
- `@mul`
  Marks the method as the `*` operator overload.
- `@div`
  Marks the method as the `/` operator overload.
- `@mod`
  Marks the method as the `%` operator overload.
- `@neg`
  Marks the method as the unary `-` operator overload.
- `@pos`
  Marks the method as the unary `+` operator overload.
- `@bitAnd`
  Marks the method as the `&` operator overload.
- `@bitOr`
  Marks the method as the `|` operator overload.
- `@bitXor`
  Marks the method as the `^` operator overload.
- `@bitNot`
  Marks the method as the `~` operator overload.
- `@at`
  Marks the method as the `[]` operator overload.
- `@call`
  Marks the method as the `()` operator overload.
- `@eq`
  Marks the method as the `==` operator overload.
- `@greater`
  Marks the method as the `>` operator overload.
- `@lower`
  Marks the method as the `<` operator overload.
  
## Class attributes
- `@stack`
  Makes the object always stack-allocated, no matter it's size.

