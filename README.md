# Romang
Make "programs" using roman numerals

## Guide
Similar to Brainfuck, Romang has a 30.000 8-bit array, with values initialized at 0. These values can be modified using the following operators:

```
i - add 1 to the current pointer
v - add 5 to the current pointer
x - add 10 to the current pointer
l - add 50 to the current pointer
c - add 100 to the current pointer
. - print the pointer's value (as a character)
> - move to the next pointer
```

Any other character will be ignored

## Usage
```
$ rustc romang.rs
$ ./romang hi.rm
```

## Example
```
ciiii.>cv.>xxxiii.
```

This program prints "hi!"

## Coming soon
[Run this monstruosity on the web](https://catmanl.github.io/romang/index.html)

