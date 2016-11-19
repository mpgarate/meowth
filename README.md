# Meowth Pokemon Language Interpreter

## Primitive Types

```
pokemon [0-9]+
battle win|lose
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
bike b = 5;
b = 3;
pokedex(b); // 3
pokedex(back(b)); // 3
pokedex(b); // 5
```

declare an attack to reuse meowth expressions
```
attack gnaw(n) {
  return n - 10;
};

pokeball kabutops = gnaw(mew);
pokedex(kabutops); // 141
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
