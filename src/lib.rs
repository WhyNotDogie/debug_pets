/// Debug pets is a crate that adds the cutest debugging tools you have ever seen.
/// 
/// Example usage:
/// 
/// ```
/// use debug_pets::prelude::*;
/// 
/// static DOG: PetDebugger<pets::Dog, DefaultPrinter> = PetDebugger::new();
/// 
/// fn main() {
///     DOG.got_here("main function")
/// }
/// ```
/// 
/// Console output can vary. Possible console output:
/// 
/// ```txt
/// $ cargo run
/// Barked at main function!
/// ```
/// 
/// Contains many other animals for you to explore!

pub mod prelude;
pub mod pets;
pub mod printer;

use std::{marker::PhantomData, ops::Deref, sync::LazyLock};

use printer::Printer;

pub struct PetDebugger<P: Pet, R: Printer> {
    lazy: LazyLock<OpenDebugger<P>>,
    _p: PhantomData<R>
}

impl<P: Pet, R: Printer> PetDebugger<P, R> {
    pub const fn new() -> Self {
        Self {
            lazy: LazyLock::new(|| OpenDebugger { pet: P::new() }),
            _p: PhantomData
        }
    }

    pub fn got_here<T: Into<String>>(&self, location: T) {
        R::print(&self.dailog_got_here(&*location.into()));
    }
}

impl<P: Pet, R: Printer> Deref for PetDebugger<P, R> {
    type Target = OpenDebugger<P>;
    fn deref(&self) -> &Self::Target {
        return &*self.lazy;
    }
}

pub struct OpenDebugger<P: Pet> {
    pet: P
}

impl<P: Pet> OpenDebugger<P> {
    pub fn dailog_got_here(&self, location: &str) -> String {
        self.pet.dialog_got_here(location)
    }
}

pub trait Pet {
    fn dialog_got_here(&self, location: &str) -> String;
    fn new() -> Self;
}