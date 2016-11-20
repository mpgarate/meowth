# Meowth: Pokemon Language Interpreter

## How to run
1. [Install stable Rust](https://www.rust-lang.org/en-US/downloads.html)
2. `git clone https://github.com/mpgarate/meowth.git && cd meowth`
3. Access the REPL with `cargo run`

## Primitive Types

```
// pokemon [0-9]+
// battle win|lose

meowth :: win ? 111 : 222
 => 111
meowth :: lose ? 111 : 222
 => 222
```


## Variable bindings

A `pokeball` is a [container](https://www.youtube.com/watch?v=kXSXLQOcmeA) that can store pokemon or battle values. You can think of it like a const binding. 

```
meowth :: pokeball mew = 151;
 => ()
meowth :: pokeball pikachu = 25;
 => ()
```
Use a `pokedex()` [get information](https://www.youtube.com/watch?v=He1g6IZBUE0) on a pokemon or battle. The result is printed to stdout. 

```
meowth :: pokedex(mew);
151
 => ()
```

Most pokemon can only [say their own name](https://www.youtube.com/watch?v=7O9SSHU0zt8). Use `speak()` to print a bound variable name. 

```
meowth :: speak(pikachu);
pikachu
 => ()
```
`bike` is a variable binding which can be borrowed. The original value can be [given back someday](https://www.youtube.com/watch?v=GMJuSajbT40) (but usually isn't). It behaves like a stack.

```
meowth :: bike b = 5;
 => ()
meowth :: b = 3;
 => ()
meowth :: b   
 => 3
meowth :: give(b)
 => 3
meowth :: b
 => 5
```

An `attack` is a [powerful](https://www.youtube.com/watch?v=UNOxXu9m4m4&t=0m35s) construct for reusing meowth expressions. 
```
meowth :: pokeball mew = 151;
 => ()
meowth :: attack gnaw(n) { n - 10 };
 => ()
meowth :: pokeball kabutops = gnaw(mew);
 => ()
meowth :: kabutops    
 => 141
meowth :: gnaw(kabutops)
 => 131
```

## Control flow
```
battle (pikachu beats mew) {
  pokedex(pikachu);
} rebattle (mew draws pikachu) {
  pokedex(mew);
  pokedex(pikachu);
} rebattle (mew survives pikachu) {
  pokedex(mew);
} run {
  pokedex(pikachu);
}
```
Does something like this:
```
if (pikachu > mew) {
  print(pikachu);
} else if (mew == pikachu) {
  print(mew);
  print(pikachu);
} else if (mew >= pikachu) {
  print(mew);
} else {
  print(pikachu);
}
```

Sometimes it is useful to repeat an expression. For this we can use a defend loop.
```
meowth :: bike i = 0;
 => ()
meowth :: defend (10 beats i) { i = i + 1; pokedex(i); };
1
2
3
4
5
6
7
8
9
10
 => ()
```

## About
Meowth is a hack based on [boxx](https://github.com/mpgarate/boxx).
