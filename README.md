#### 🧵 Loom
... is a friendly, lightweight programming language for math and games

#### 💡 Status
`Loom` is unstable, and currently a **work in progress project (WIP)**.  
Nightly builds are available giving you a chance to explore and test the language as it evolves.

#### 🏠 Quick Start
```loom
println("Hello, world!");
```

#### ✨ Examples
```loom
fun ask_name() {
  putln("👋 Hey there! What's your name?")
  return readln()
}

fun greet(name) {
  greetings := [
    "Nice to meet you",
    "Welcome aboard",
    "Glad you're here",
    "Hey"
  ]

  greeting := greetings.choice()
  putln("🎉 " + greeting + ", " + name)

  if len_of(name) < 3 {
    putln("That's a pretty short name 😄")
  } else if len_of(name) > 10 {
    putln("Wow, that's a long name 😳")
  } else {
    putln("Cool name 👍")
  }
}

name := ask_name()
greet(name)
```

```loom
class Sandwich {
  fun init(self, cheese, tomatoes) {
    self.cheese := cheese
    self.tomatoes := tomatoes
  }

  fun cook(self) {
    putln(
      "Sandwich is ready with "
      + str_of(self.cheese) + " cheese, "
      + str_of(self.tomatoes) + " tomatoes."
    )
  }
}

sandwich := Sandwich(3, 2)
sandwich.cook()
```

#### 🔦 ToDo
- [ ] `os` library
- [x] `fs` library
- [x] `time` library
- [x] update `math` library
- [x] implement `random` in `math` library
- [x] implement `choice` for list
- [ ] `strings` library
- [ ] `fmt` library
- [x] `convert` library
- [ ] `reflect` library
- [x] `env` library (by `Antares64`)
- [x] `mem` library
- [ ] `unsafe` library
- [ ] `signal` library
- [ ] `zip` library
- [ ] `color` library
- [ ] `ffi` library
- [ ] `net/http` library
- [ ] `net/tcp` library
- [ ] `net/udp` library
- [x] `crypto` library
- [ ] `sys` library
- [x] `process` library
- [x] implement anonymous functions
- [x] extend for loop to iterate over items of list
- [x] implement dictionaries
- [ ] write specification for standard library
- [ ] start writing documentation
- [ ] basic jit
