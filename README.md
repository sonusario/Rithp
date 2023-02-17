# Rithp
## A Rust made LiTHp

I'm just messing around with ideas for a Lisp interpreter.

Example expressions and their results if the interpreter follows my internal spec. The "->" indicates an intermediate step while "=>" indicates a final step.
```
() => ()
(()) => ()
(() 1 2 3) => (1 2 3)
(+ 1 2 3) => 6

(2 3 (+ 4 5)) -> (2 3 9) => 3 9 3 9

(() 2 3 (+ 4 5)) -> (() 2 3 9) => (2 3 9)

(+ 2 3 (+ 4 5)) -> (+ 2 3 9) => 14

(+ 2 3 (4 5)) -> (+ 2 3 5 5 5 5) => 25

(+ 2 3 (() 4 5)) -> (+ 2 3 (4 5)) => (9 10)

(* 2 3) => 6

(* 3 2) => 6

(* (3 2)) -> (* 2 2 2) => 8

(* (2 3)) -> (* 3 3) => 9
```

## Core ideas involve:
0. Using "()" for quote/quasiquote: only quotes the immediate list, sub-lists are unquoted
0. Using numbers as operators: No real reason, just seemed cool on first thought
0. Including errors as part of the environment map
0. List destructuring, eliminating atom, first/car, rest/cdr, cons, and if/cond functions
0. Implicit maping in some cases
0. etc.?
