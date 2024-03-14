#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use bevy::prelude::*;

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn run() {
    App::new().run();
}
