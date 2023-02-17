# Rithp
## A Rust made LiTHp

I'm just messing around with ideas for a Lisp interpreter.

Example expressions and their results if the interpreter follows my internal spec. The "->" indicates an intermediate step while "=>" indicates a final step.
```
() // => ()
(()) // => ()
(() 1 2 3) // => (1 2 3)
(+ 1 2 3) // => 6

(2 3 (+ 4 5)) // -> (2 3 9) => 3 9 3 9

(() 2 3 (+ 4 5)) // -> (() 2 3 9) => (2 3 9)

(+ 2 3 (+ 4 5)) // -> (+ 2 3 9) => 14

(+ 2 3 (4 5)) // -> (+ 2 3 5 5 5 5) => 25

(+ 2 3 (() 4 5)) // -> (+ 2 3 (4 5)) => (9 10)

(* 2 3) // => 6

(* 3 2) // => 6

(* (3 2)) // -> (* 2 2 2) => 8

(* (2 3)) // -> (* 3 3) => 9
```

## Core ideas involve:
1. Using "()" for quote/quasi-quote: only quotes the immediate list, sub-lists are unquoted
1. Using numbers as operators: No real reason, just seemed cool on first thought
1. Using "_" as a don't care value
1. Including errors as part of the environment map
1. List destructuring, eliminating atom, first/car, rest/cdr, cons, and if/cond functions
1. Implicit mapping in some cases
1. Reversible functions, which query in the case of a one to many output mapping 
1. etc.?

---

Work in progress syntax ideas
```

(+ 2) => 2 //implicit identity addition
(* 3) => 3 //implicit identity multiplication

name::                    // a reference to var/function "name"
:type<;(type;|bound;)*>>: // a reference to type "type"
::value                   // a value

name:type:value // e.g. x:i32:-3

// Define variable reference "q" with bounded type
(define mut q:Number;4..=11:5) //given q, yield 5
(into q 12) // (:Err: Cannot assign 12 into q as q is a Number bounded from 4 up to and including 11

(+ 3 (q)) // -> (+ 3 5) => 8

// Define function reference "abs" returning bounded number type
(define (abs:Number->Number;>=0: x:Number:)
 ((abs x:>=0:) x)
 (else (* x -1))
)

// Define non negative multi product "!-m*" function reference; type not specified. 
// most cases could be avoided my specifying x's type, but are shown for demonstrative purpose
(define (!-m*:: x::)                                                          // call format then implicit cond
 ((!-m* ()) 0)                                                                // where x is an empty list, yield 0: a terminating case
 ((!-m* (first:Number:)) first)                                               // where x is a one item list, yield that item if it is not a list: a terminating case
 ((!-m* (::0 rest::)) 0)                                                      // where x is a list and the first item is 0, yield 0: a terminating case
 ((!-m* (first:List: rest::)) (m* (((lambda (-> y) ((-> (lst::)) lst)) first) rest)) // flatten first: a recursive case
 ((!-m* (first:: second:List: rest::)) (m* (first ((lambda (-> y) ((-> (lst::)) lst)) second) rest)) // flatten second: a recursive case
 ((!-m* (first:: second:: rest::) (m* (* first second) rest)) //multiply first and second, then repeat: a recursive case
 ((!-m* :!Number:) (:Err: Not a number))                                      // return error type where x is not a number
 ((!-m* x) (m* (x))                                                           // restructure into a list where item(s) after "m*" is(are) not collectively in a list: a recursive case
)

// Define using lambda
// Skip the combinator: Lambdas in Rithp could perform recursion using
//     "->" and can be labeled for nesting via "label->"
// Obviously if you a generating a function variable with a lambda you can
//     just use the variable name
(define factorial:Number->Number:(lambda (fac-> n:Number;>=0:)
 ((fac-> ::0) 1)
 (else (* n (fac-> (- n 1))))
))


//Could also be used for type conversion
(define (:u32: x:Number;>=0:))


```