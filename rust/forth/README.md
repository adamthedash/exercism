# Forth

Welcome to Forth on Exercism's Rust Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## Instructions

Implement an evaluator for a very simple subset of Forth.

[Forth][forth]
is a stack-based programming language.
Implement a very basic evaluator for a small subset of Forth.

Your evaluator has to support the following words:

- `+`, `-`, `*`, `/` (integer arithmetic)
- `DUP`, `DROP`, `SWAP`, `OVER` (stack manipulation)

Your evaluator also has to support defining new words using the customary syntax: `: word-name definition ;`.

To keep things simple the only data type you need to support is signed integers of at least 16 bits size.

You should use the following rules for the syntax: a number is a sequence of one or more (ASCII) digits, a word is a
sequence of one or more letters, digits, symbols or punctuation that is not a number.
(Forth probably uses slightly different rules, but this is close enough.)

Words are case-insensitive.

[forth]: https://en.wikipedia.org/wiki/Forth_%28programming_language%29

Note the additional test case in `tests/alloc-attack.rs`. It tests against
algorithmically inefficient implementations. Because of that, it usually times
out online instead of outright failing, leading to a less helpful error message.

## Strategy

1) Chop string into tokens


2) Parse the tokens
    1) If we hit a digit or operator, do nothing
    2) If we hit a variable name
        1) If it is a known name, replace the token with the sub-token list
        2) If it is unknown
            1) if it is a keyword, do nothing
            2) else throw an unknown token error
    3) If we hit :
        1) Store the following token as the lookup key.
            1) If it is an operator or digit, throw an invalid word error
        2) Accumulate the sub-tokens until we hit ;
        3) Parse the sub-tokens (recursive)
        4) Store the result as a variable


3) Evaluate the tokens
    1) If we hit a digit, add it to the stack
    2) If we hit a boolean operator
        1) If stack size is < 2, throw stack underflow error
        2) Pop top 2 from stack
        3) Execute operator
        4) Push result to stack
    3) If we hit a keyword
        1) If DUP
            1) If stack size is <1, throw underflow
            2) else duplicate item on top of stack
        2) If DROP
            1) If stack size is <1, throw underflow
            2) else drop item on top of stack
        3) If SWAP
            1) If stack size is <2, throw underflow
            2) Swap top 2 items on stack
        3) If OVER
            1) If stack size is <2, throw underflow
            2) Duplicate 2nd top item on stack

## Source

### Created by

- @EduardoBautista

### Contributed to by

- @ashleygwilliams
- @bobahop
- @coriolinus
- @cwhakes
- @EduardoBautista
- @efx
- @ErikSchierboom
- @gefjon
- @gsauthof
- @IanWhitney
- @kytrinyx
- @lutostag
- @N-Parsons
- @nfiles
- @PaulDance
- @petertseng
- @pminten
- @robkeim
- @rofrol
- @stevejb71
- @stringparser
- @xakon
- @ZapAnton