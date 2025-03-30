# Boolean algebra evaluator
Just a simple evaluator/interpretter based on an AST

# Build
- just clone the repo
- `cd` there
- `cargo build`

# Syntax
> [!IMPORTANT]
> All characters in an expression must be lower case
### Operators
>| Operator | Character |
>|----------|-----------|
>| AND      | &         |
>| OR       | \|        |
>| NOT      | !         |
>| XOR      | ^         |
### Literals
>| Literal | Value |
>|---------|-------|
>| true    | true  |
>| 1       | true  |
>| false   | false |
>| 0       | false |
### Groups
- Any expression can be wrapped in `(...)` to make a group
### Identifiers
- `a-z` are identifiers if they are not part of a literal
- each identifier can be true or false
- identifiers are placeholders that get a value during evaluation
- they resemble the state of an hypotetical bus
- the evaluator handles them not strictly so if you use `a&c&e&` `a` will be the first bit, `c` the second and `e` the third so there will overall be still only `7` variants not a..e (1-5) `63` variants
- during truth table generation each identifier combination gets evaluated

# Operator Prioritys
| Priority | Operator  | Symbol(s)                               |
|----------|-----------|-----------------------------------------|
| Highest  | NOT       | ¬, ̄, !                                  |
|          | AND       | ∧, ·                                   |
|          | OR        | ∨, +                                   |
|          | XOR       | ⊕, ⊻                                   |
| Lowest   | EQUALS    | =, ≡, ↔, ⇔                             |

# Usage
#### booleval --help
> Prints the help
#### booleval [expression]
> Evaluates a pure bool expression, no identifiers, for example "true^false" 
#### booleval -T [expression] {-t -f}
> Evaluates all posible combinations of expression and prints it as a truth table
> You can optionally add a `-f` XOR `-t` flags to filter for `-f=false`, `-t=true` results only
```bash
> .\booleval -T "a^b"
╭───────┬───────┬────────╮
│ b     │ a     │ Result │
├───────┼───────┼────────┤
│ false │ false │ false  │
│ false │ true  │ true   │
│ true  │ false │ true   │
│ true  │ true  │ false  │
╰───────┴───────┴────────╯
```
#### booleval -t [...args] [expression]
> Evaluates the specefied expression with a specified identifier state
```bash
> booleval -t 111 "a&b&c"
true
# ...args = binary string each bit mapping to 1 identifier (a = 1, b = 1, c = 1) = 111

> booleval -t 7 "a&b&c"
true
# ...args = numeric string each bit mapping to 1 identifier (a = 1, b = 1, c = 1) = 111 = 7

> booleval -t true true true "a&b&c"
#OR
> booleval -t 1 1 1 "a&b&c"
true
# ...args = boolean string each mapping to 1 bit (a = 1, b = 1, c = 1) = true true true or 1 1 1
```
#### booleval -a [expression] {-p, -e}
> Prints the ast for the boolean expression, identifiers are allowed
```bash
# Default
> booleval -a "a|b"
    |
   / \
  /   \
 /     \
a       b

# Pretty
> booleval -a "a|b" -p
 |
┌┴┐
a b

# Extended
> booleval -a "a|b" -e
   OR
   / \
  /   \
 /     \
a       b

# Extended & Pretty
> booleval -a "a|b" -ep
OR
┌┴┐
a b
```
> More Complex example: 2-4 Muliplexer `"(!a & !b & c) | (!a & b & d) | (a & !b & e) | (a & b & f)"`
> where a & b are the selector bits and c, d, e and f are the value bits
```bash
> booleval -a "(!a & !b & c) | (!a & b & d) | (a & !b & e) | (a & b & f)" -pe
                    OR
               ┌─────┴──────┐
              OR           GRP
         ┌─────┴─────┐      │
        OR          GRP    AND
     ┌───┴────┐      │     ┌┴─┐
    GRP      GRP    AND   AND f
     │        │    ┌─┴──┐ ┌┴┐
    AND      AND  AND   e a b
  ┌──┴──┐   ┌─┴─┐ ┌┴┐
 AND    c  AND  d a NOT
┌─┴─┐     ┌─┴─┐     │
NOT NOT   NOT b     b
│   │     │
a   b     a
```
