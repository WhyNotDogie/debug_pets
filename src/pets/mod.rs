use crate::Pet;

#[cfg(feature = "random")]
use rand::random_range;

use std::sync::atomic::{AtomicU8, Ordering};

macro_rules! twodialoganimal {
    ($name:ident, $dialog1:literal, $dialog2:literal) => {
        pub struct $name {
            variant: AtomicU8
        }
        
        impl Pet for $name {
            #[cfg(not(feature="random"))]
            fn dialog_got_here(&self, location: &str) -> String {
                if self.variant.fetch_add(1, Ordering::Relaxed) % 2u8 == 0 {
                    format!($dialog1, location)
                } else {
                    format!($dialog2, location)
                }
            }
            #[cfg(feature="random")]
            fn dialog_got_here(&self, location: &str) -> String {
                if self.variant.fetch_add(random_range(0u8..4u8), Ordering::Relaxed) % 2u8 == 0 {
                    format!($dialog1, location)
                } else {
                    format!($dialog2, location)
                }
            }
            fn new() -> Self {
                Self {
                    variant: AtomicU8::new(0)
                }
            }
        }
    };
}

twodialoganimal!{Cat, "Meow at {}!", "Purr at {}!"}
twodialoganimal!{Fox, "?? at {}!", "[REDACTED] at {}!"}
twodialoganimal!{Cow, "Moo at {}!", "Bell rang at {}!"}
twodialoganimal!{Pig, "Oink at {}!", "Rolling in mud at {}!"}
twodialoganimal!{Dog, "Woof at {}!", "Bark at {}!"}
twodialoganimal!{Chihuahua, "Yap at {}!", "Urf at {}!"}