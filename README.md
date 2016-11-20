# Meowth: Pokemon Language Interpreter

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

`pokeball` is a const binding

```
meowth :: pokeball mew = 151;
 => ()
meowth :: pokeball pikachu = 25;
 => ()
```
use `pokedex()` to print a value

```
meowth :: pokedex(mew);
151
 => ()
```
use `speak()` to print a bound variable name

```
meowth :: speak(pikachu);
pikachu
 => ()
```
`bike` is a variable binding which can be stolen. The original value can be given back someday (but usually isn't). It behaves like a stack.

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

declare an attack to reuse meowth expressions
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
// print the numbers 1-10

bike i = 0;
defend(10 beats i) {
  i = i + 1;
  pokedex(i);
}
```
