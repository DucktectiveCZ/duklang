# Hello world

```duk
@start
fun main() {
    writeln("Hello world!");
}
```
The `@start` attribute tells the compiler that the program starts in this function. The function doesn't have to be called `main()`. In fact, it can be an anonymous function and have no name at all!
```duk
@start fun() {
    writeln("Hello world!");
}
```

