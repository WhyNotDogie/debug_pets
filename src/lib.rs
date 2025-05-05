pub mod prelude;
pub mod pets;
pub mod printer;

use std::{marker::PhantomData, ops::Deref, sync::LazyLock};

use printer::Printer;

pub struct PetDebugger<P: Pet, R: Printer> {
    lazy: LazyLock<OpenDebugger<P>>,
    _p: PhantomData<R>
}

pub struct OpenDebugger<P: Pet> {
    pet: P
}

impl<P: Pet> OpenDebugger<P> {
    pub fn dailog_got_here(&self, location: &str) -> String {
        self.pet.dialog_got_here(location)
    }
}

impl<P: Pet, R: Printer> PetDebugger<P, R> {
    pub const fn new() -> Self {
        Self {
            lazy: LazyLock::new(|| OpenDebugger { pet: P::new() }),
            _p: PhantomData
        }
    }

    pub fn got_here(&self, location: &str) {
        R::print(&self.dailog_got_here(location));
    }
}

impl<P: Pet, R: Printer> Deref for PetDebugger<P, R> {
    type Target = OpenDebugger<P>;
    fn deref(&self) -> &Self::Target {
        return &*self.lazy;
    }
}

pub trait Pet {
    fn dialog_got_here(&self, location: &str) -> String;
    fn new() -> Self;
}