//! This module provides a layout generator that can generate Neo variant layouts
//! from given string representations of its base layer.

use core::fmt;

use crate::layout::Layout;
use anyhow::Result;

pub trait LayoutGenerator: Send + Sync + LayoutGeneratorClone + fmt::Debug {
    fn generate(&self, layout_keys: &str) -> Result<Layout>;
}

impl Clone for Box<dyn LayoutGenerator> {
    fn clone(&self) -> Box<dyn LayoutGenerator> {
        self.clone_box()
    }
}

/// Helper trait for realizing clonability for `Box<dyn UnigramMetric>`.
pub trait LayoutGeneratorClone {
    fn clone_box(&self) -> Box<dyn LayoutGenerator>;
}

impl<T> LayoutGeneratorClone for T
where
    T: 'static + LayoutGenerator + Clone,
{
    fn clone_box(&self) -> Box<dyn LayoutGenerator> {
        Box::new(self.clone())
    }
}
