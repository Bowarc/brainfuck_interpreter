## Brainfuck interpreter in pure rust

This was a fun exercice and will probably don't receive more updates


### Usage
```console
cargo r -- <path to brainfuck script>
```

If you want to run the mandelbrot example, you should use the `--release` cargo option to make the execution faster.

The fibonacci example is an infinite loop, you'll have to CTRL+C it.

### Known issues:

The input system is bad  
There is no comment filtering, meaning that `[hi]` is taken as brainfuck code and the letters are ignored


### Credit:
Most of the examples in ./scripts are from <http://www.brainfuck.org/> or <https://en.wikipedia.org/wiki/Brainfuck>
