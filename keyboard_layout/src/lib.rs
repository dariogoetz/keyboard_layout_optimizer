//! The `keyboard_layout` crate provides basic objects for representing a keyboard
//! and layouts that can be realized on it. These objects are analysed in the `layout_evaluation`
//! crate in terms of their performance and comfort.
//!
//! The core object is the [`layout::LayerKey`] which represents a symbol that can be generated
//! with a given layout. It provides data about the involved key properties, required modifiers,
//! and other associated properties.

pub mod config;
pub mod grouped_layout_generator;
pub mod key;
pub mod keyboard;
pub mod layout;
pub mod layout_generator;
pub mod neo_layout_generator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
