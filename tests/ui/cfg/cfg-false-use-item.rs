//! Test that use items with cfg(false) are properly filtered out

//@ run-pass

pub fn main() {
    // Make sure that this view item is filtered out because otherwise it would
    // trigger a compilation error
    #[cfg(false)] use bar as foo;
}
