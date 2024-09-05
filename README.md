# Primal - an even more basic kin of the BASIC language

It's just really a tiny BASIC-like language compiler written in Rust.

I don't know what the output language will be... yet. 

## Used material

This reference is also for myself :) I tend to forget things sometimes...
1. Austin Henley
    - [challenging projects - tinyBASIC](https://austinhenley.com/blog/challengingprojects.html)
    - [let's build a compiler](https://compilers.iecc.com/crenshaw/)
2. Wikipedia
    - [BASIC langauge](https://en.wikipedia.org/wiki/BASIC)
    - [Lexical analysis](https://en.wikipedia.org/wiki/Lexical_analysis)
    - [Syntactic analysis](https://en.wikipedia.org/wiki/Parsing)
    - [Recursive descent parsing](https://en.wikipedia.org/wiki/Recursive_descent_parser)
    - [Abstract syntax tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
    - [Semantic analysis](https://en.wikipedia.org/wiki/Semantic_analysis_(compilers))
    - [Optimization passes](https://en.wikipedia.org/wiki/Optimizing_compiler)
    - [Code generation](https://en.wikipedia.org/wiki/Code_generation_(compiler))

## Project structure

It's split in two, binary and library, parts.

### Library part

Create a working rust code.

### Binary (execute) part

Execute the code created with the library.

> NOTE:
> Use the shell script `primal.sh` to run a desired file:
```bash
bash primal.sh test.roq
# Or give it +x permissions and simply execute
./primal.sh test.roq
```

## Grammar-tree

See [grammar-tree.txt](./grammar-tree.txt)

## Keywords (planned or already implemented)

| Keyword | Desc | Showcase | Options |
| --------------- | --------------- | --------------- | --------------- |
| LET | define a variable | `LET x = 5` | multiple types available |
| INPUT | ask user for input (number) | `INPUT "How much cheese?" user_guess` | multiple types available |
| IF ... THEN? | control flow | `IF true THEN` / `IF x == 5 THEN ... ENDIF` | - |
| WHILE ... FINISH / END | Basic loops | `WHILE x > 10 DO ... ENDWHILE` | - |
| PRINT | print valiables or text or else | `PRINT "Hello, World!"` | Can be a variable |

Not likely to be implemented
| Keyword | Desc | Showcase | Options |
| --------------- | --------------- | --------------- | --------------- |
| LIST | output a [quine](https://en.wikipedia.org/wiki/Quine) =D | `LIST` | - | - |

## Other functionality

### Basic arithmetics
> NOTE: Math must work on numbers and/or variables

Including:
- addition
- substraction
- multiplication
- division
- modulo
- exponents?
- log(n)s?
- roots?

### Comments

Commenting is done with the hash `#` sign. The compiler should throw it away.
Comments apply till the end of the line

```
PRINT "Hi mom!" # This is a comment LET x = 1 - it doesn't work here
```

### Status

MVP (Minimum Viable Product) state reached.
Now is the time for some upgrades!

