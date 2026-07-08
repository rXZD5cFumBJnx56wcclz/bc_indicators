#![allow(unused_imports)]
pub use std::sync::LazyLock;

pub use bc_utils_lg::statics::prices::{CLOSE, CLOSE_LAST, HIGH, LOW, OPEN, OPEN_LAST};
pub use criterion::{Criterion, criterion_group, criterion_main};

pub use bc_indicators::main_trait::{Indicator, IndicatorExt};
