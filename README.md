Debug pets is a crate that adds the cutest debugging tools you have ever seen.

Example usage:

```rust
use debug_pets::prelude::*;

static DOG: PetDebugger<pets::Dog, DefaultPrinter> = PetDebugger::new();

fn main() {
    DOG.got_here("main function")
}
```

Console output can vary. Possible console output:

```txt
$ cargo run
Barked at main function!
```

Contains many other animals for you to explore!