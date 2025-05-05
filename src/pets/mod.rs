use crate::Pet;

#[cfg(feature = "random")]
use rand::random_range;

use std::sync::atomic::{AtomicU8, Ordering};

pub struct Dog {
    variant: AtomicU8
}

impl Pet for Dog {
    #[cfg(not(feature="random"))]
    fn dialog_got_here(&self, location: &str) -> String {
        if self.variant.fetch_add(1, Ordering::Relaxed) % 2u8 == 0 {
            format!("Woofed at {}!", location)
        } else {
            format!("Barked at {}!", location)
        }
    }
    #[cfg(feature="random")]
    fn dialog_got_here(&self, location: &str) -> String {
        if self.variant.fetch_add(random_range(0u8..4u8), Ordering::Relaxed) % 2u8 == 0 {
            format!("Woofed at {}!", location)
        } else {
            format!("Barked at {}!", location)
        }
    }
    fn new() -> Self {
        Self {
            variant: AtomicU8::new(0)
        }
    }
}

pub struct Cat {
    variant: AtomicU8
}

impl Pet for Cat {
    #[cfg(not(feature="random"))]
    fn dialog_got_here(&self, location: &str) -> String {
        if self.variant.fetch_add(1, Ordering::Relaxed) % 2u8 == 0 {
            format!("Meowed at {}!", location)
        } else {
            format!("Purred at {}!", location)
        }
    }
    #[cfg(feature="random")]
    fn dialog_got_here(&self, location: &str) -> String {
        if self.variant.fetch_add(random_range(0u8..4u8), Ordering::Relaxed) % 2u8 == 0 {
            format!("Meowed at {}!", location)
        } else {
            format!("Purred at {}!", location)
        }
    }
    fn new() -> Self {
        Self {
            variant: AtomicU8::new(0)
        }
    }
}

pub struct Fox {
    variant: AtomicU8
}

impl Pet for Fox {
    #[cfg(not(feature="random"))]
    fn dialog_got_here(&self, location: &str) -> String {
        if self.variant.fetch_add(1, Ordering::Relaxed) % 2u8 == 0 {
            format!("?? at {}!", location)
        } else {
            format!("?? at {}!", location)
        }
    }
    #[cfg(feature="random")]
    fn dialog_got_here(&self, location: &str) -> String {
        if self.variant.fetch_add(random_range(0u8..4u8), Ordering::Relaxed) % 2u8 == 0 {
            format!("?? at {}!", location)
        } else {
            format!("?? at {}!", location)
        }
    }
    fn new() -> Self {
        Self {
            variant: AtomicU8::new(0)
        }
    }
}

pub struct Cow {
    variant: AtomicU8
}

impl Pet for Cow {
    #[cfg(not(feature="random"))]
    fn dialog_got_here(&self, location: &str) -> String {
        if self.variant.fetch_add(1, Ordering::Relaxed) % 2u8 == 0 {
            format!("Moo at {}!", location)
        } else {
            format!("Bell rang at {}!", location)
        }
    }
    #[cfg(feature="random")]
    fn dialog_got_here(&self, location: &str) -> String {
        if self.variant.fetch_add(random_range(0u8..4u8), Ordering::Relaxed) % 2u8 == 0 {
            format!("Moo at {}!", location)
        } else {
            format!("Bell rang at {}!", location)
        }
    }
    fn new() -> Self {
        Self {
            variant: AtomicU8::new(0)
        }
    }
}