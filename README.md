# MathLang

MathLang is toy programming language for simple mathemmatical calculations. We largely follow the text from Creative Projects for Rust Programmers from Packt Publishing.

## Usage

First install Rust. You can then use cargo to build the interpreter:

```
$cd math_lang && cargo build
```

Finally you can run like so:

For the REPL,
```
$./math_lang/target/debug/math_lang.exe --repl
```
or 

```
$cargo run --repl
``` 
 

To compile MathLang to Rust,

```
$./math_lang/target/debug/math_lang.exe --compile <path/to/.math/file>
```

or

```
$cargo run --compile <path/to/.math/file>
```

To interpret MathLang,

```
$./math_lang/target/debug/math_lang.exe --interpret "program to interpret"
```

or

```
$cargo run --interpret "program to interpret"
```


## Example of a MathLang file

```
// example.math
var a = 1
var b
b = a + 2
out b *(a^2 mod 3) - 4 

```