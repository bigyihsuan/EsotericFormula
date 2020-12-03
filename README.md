# Esoteric Reaction

A stack-based/functional esolang using chemical "formulas" as code. The name is a pun on "exothermic reaction".

## Types

Number: A real number.

* When used with list indexing, only the integer part is used.
* Characters are represented as their UTF-8/ASCII value in the integer part.

List: an infinitely-nestable heterogenous list that can hold values.

* Lists consisting of solely integer values can be considered as Strings.
* Lists can contain lists.
* Lists can be of any length.

## Elements

Each of the 118 elements of the periodic table represents a different function/instruction.

Coefficients indicate that element being repeated several times. So `2O == O + O`.

Subscripts change the number of arguments expected for the function. So `Li_3` expects 3 arguments following it.

Strings of consecutive elements (`H_2O`, `NLi_4`, etc) represents function composition from right to left. So, `H_2O` is `O(H_2())`.

### Undiscovered Elements

Undiscovered elements (atomic number N >= 119, `Uue`) are available as bindable names. Use `=` equations to bind functions to a name.

Undiscovered elements must satisfy the following:

* The first character is uppercase, while all other characters are lowercase.
* When converting to an atomic number `N`, satisfies `N >= 119`.
* Only contains the following characters (from 0-9):
  * `nubtqphsoe`

*TODO: add rules on how to make element names here*

### Light and Heat

`light` and `heat` are special elements that handle STDIO. `light` is input, and `heat` is output.

#### Light

Light allows programs to take input from STDIN.

* `light`: Take 1 byte from STDIN and push it onto the stack.
* `light_N`: Take `N` bytes from STDIN and push it as a list of numbers onto the stack.

#### Heat

Heat allows programs to output to STDOUT.

* `heat`: Pop 1 element from the stack and output it to STDOUT.
* `heat_N`: Pop `N` elements from the stack and output them, in order, to STDOUT.

## Equations

Equations are always of the following grammar:

```
TODO
```

Comments are signified by a semicolon `; comment up to the end of the line`.

Equations are chemicals separated by an equation symbol.

The equation symbol can be one of the following two symbols:

* `=`: Definition. Bind a name (product) to functions (reagents). The name is a systematic element name (see above).
* `->`: Execute the preceeding function calls.
  * Both sides of the equation must follow the Law of Conservation of Mass. `light` and `heat` are exempt from this restriction.

Equations consist of terms spearated by `+` signs. Each term consists of molecules or elements. Both can be prepended by a coefficient `N`, which is syntactic sugar for `N` of those terms appearing. (`2H_2` is the same as `H_2 + H_2`.)

Each element can have a subscript `_N` appended, representing a differing number of arguments. `H` takes either 0 or 1 arguments, while `H_2` takes 2. Arguments are filled through additional terms.

Elements concatenated together are molecules. This represents a composition of the elements from left to right:

```
CNH + A -> C(N(H(A)))
CO_2 + A + B -> C(O(A,B))
C_3H_6 + X + Y + 6Z
    -> C_2H_6 + X + Y + Z + Z + Z + Z + Z + Z
    -> C(X,Y,H(Z,Z,Z,Z,Z,Z))
```

## Example Equations

### Cat

```
light + heat + Uue = Uue ; Declare a recursive cat, 1 byte at a time forever.
Uue -> Uue               ; Call cat.
```
