# Esoteric Reaction

A stack-based/functional/concatenative? esolang using chemical "formulas" as code. The name is a pun on "exothermic reaction".

## Types

Number: A real number.

* When used with list indexing and function referencing, only the integer part is used.
* Characters are represented as their UTF-8/ASCII value in the integer part.

List: An infinitely-nestable heterogenous list that can hold values.

* Lists consisting of solely integer values can be considered as Strings.
* Lists can contain lists.
* Lists can be of any length.

Function: A function reference, used primarily in higher-order functions, but in some control flow instructions as well.

## Elements

Each of the 118 elements of the periodic table represents a different function/instruction.

Coefficients indicate that element being repeated several times. So `2O -> O + O`.

Subscripts change the number of arguments expected for the function. So `Li_3` expects 3 arguments following it.

Strings of consecutive elements (`H_2O`, `NLi_4`, etc) represents function composition from right to left. So, `H_2O` is `O(H_2())`.

### Undiscovered Elements

Undiscovered elements (atomic number N >= 119, `Uue`) are available as bindable names. Use `=` equations to bind functions to a name.

Undiscovered elements must satisfy the following:

* The first character is uppercase, while all other characters are lowercase.
* When converting to an atomic number `N`, satisfies `N >= 119`.
* Only contains the following characters (from 0-9):
  * `nubtqphsoe`

To make a name of atomic number `N`, do the following pseudocode:

```python
For each digit,
  append its letter value:
    n -> 0, u -> 1, b -> 2, t -> 3, q -> 4,
    p -> 5, h -> 6, s -> 7, o -> 8, e -> 9
Capitalize the first letter.
```

### Grouped Elements

Grouped elements are elements defined in a `=` equation and enclosed in parentheses `()`. Any arbitrary string can be placed inside, and are considered a single element. Do note you will still need to make sure that the equations are still balanced.

```lisp
S + To + P = (stop)   ; create grouped element (stop)
2(stop)_2 -> 4S + 4ToP ; call (stop)
```

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

```bnf
<equation> ::= <reagent> <rhs> | <COMMENT> | <reagent> <rhs> <COMMENT>
<rhs>      ::= "=" <NAME> | "->" <reagent>
<reagent>  ::= <term> | <term> "+" <reagent>
<term>     ::= <molecule> | <NUMBER> <molecule>
<molecule> ::= <atom> | <atom> <molecule>
<atom>     ::= <ELEMENT> | <name> "_" <NUMBER>
<NUMBER>   ::= integer > 0
<ELEMENT>  ::= valid element
<NAME>     ::= valid systematic chemical symbol
<COMMENT>  ::= ";" text "\n"
```

Comments are signified by a semicolon `; comment up to the end of the line`.

Equations are chemicals separated by an equation symbol. The equation symbol can be one of the following two symbols:

* `=`: Definition. Bind a name (product) to functions (reagents). The name is a systematic element name (see above).
* `->`: Execute the preceeding function calls.
  * Both sides of the equation must follow the Law of Conservation of Mass. `light` and `heat` are exempt from this restriction.

Equations consist of terms spearated by `+` signs. Each term consists of molecules or elements. Both can be prepended by a coefficient `N`, which is syntactic sugar for `N` of those terms appearing. (`2H_2` is the same as `H_2 + H_2`.)

Each element can have a subscript `_N` appended, representing a different function.

Elements concatenated together are molecules. This represents a composition of the elements from left to right:

```asm
CNH + A -> C(N(H(A)))
CO_2 + A + B -> C(O(A,B))
C_3H_6 + X + Y + 6Z
    -> C_2H_6 + X + Y + Z + Z + Z + Z + Z + Z
    -> C(X,Y,H(Z,Z,Z,Z,Z,Z))
```

### Law of Conservation of Mass

All `->` equations must follow the Law of Conservation of Mass, where both product and reagent must contain the same number and type of atoms. Light and heat are exempt from this law. If the law is found to be volated, it is considered a syntax error and parsing will stop.

```lisp
2H_2 + O_2 = (water)
(water) -> 2H_20
Na + Cl -> NaCl
Na + Cl -> Na + Cl
```

## Recursion

Recursion to an arbitrary depth is possible in Esoteric Reaction. This is done through naming and calling a function in its body, in this case in its reagents.

```lisp
CoDe + (rec) = (rec) ; recursive declaration
(rec) -> Co + De + (rec) ; function call
```

## Truthiness

In Esoteric Reaction, truthiness is defined as follows:

| Truthy      | Falsy      |
| ----------- | ---------- |
| Non-zero    | Zero       |
| Filled list | Empty list |
| Code        | -          |

## The Stack

Esoteric Reaction relies on 2 stacks: a call stack and a data stack. Because of Esoteric Reaction's ordering of code, a call stack is needed to allow for the execution of earlier function calls after later ones.

Let's consider the factorial implementation, seen below.

## Example Equations

### Cat

```lisp
light + heat + Uue = Uue ; Declare a recursive cat, 1 byte at a time forever.
Uue -> Uue               ; Call cat.
```

### Factorial

```lisp
; Factorial (n -- n!)
; construct list n to 1, then reduce multiply
Dy + Li + SbH_2O_1 
; (n [n] -- )

```
