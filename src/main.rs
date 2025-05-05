use debug_pets::prelude::*;

static DOG: PetDebugger<pets::Dog, DefaultPrinter> = PetDebugger::new();

fn main() {
    for _ in 1..10 {
        DOG.got_here("cheese");
    }
}