# MathLang

MathLang is toy programming language for simple mathemmatical calculations. We largely follow the text from Creative Projects for Rust Programmers from Packt Publishing.

## Usage

First install Rust. You can then use cargo to build the interpreter:

```
$cd math_lang_analyzer && cargo build
```

Finally you can run like so:
```
$./math_lang_analyzer/target/debug/math_lang_analyzer.exe <path/to/.math/file>
```

## Example of a MathLang file

```
// example.math
var a
a = 1
var b
b = a + 2
```