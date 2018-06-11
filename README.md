# genotype-rs
[![Build Status](https://travis-ci.org/DomWilliams0/genotype-rs.svg?branch=master)](https://travis-ci.org/DomWilliams0/genotype-rs)
[![Docs](https://docs.rs/genotype/badge.svg)](https://docs.rs/genotype)
[![Crates.io](https://img.shields.io/crates/v/genotype.svg)](https://crates.io/crates/genotype)

An abstraction layer between genotype and phenotype, with in-place mutation.

```rust
use genotype::{Param, ParamHolder, RangedParam};
use genotype::param_set::{ParamSet2d};
use genotype::mutation::*;

// a length in space in 1 dimension
// - can range from 1m to 20m
#[derive(Debug)]
struct Dimension(Param);

// rotation in degrees
// - can range from 0 - 360 degrees
#[derive(Debug)]
struct Rotation(Param);

// a 2d cuboid shape in space with a rotation
#[derive(Debug)]
struct Shape {
    dimensions: ParamSet2d<Dimension>,
    rotation: Rotation,
}

// implement RangedParam for each parameter
impl RangedParam for Dimension {
    fn range(&self) -> (Param, Param) {
        (1.0, 20.0)
    }

    // necessary boilerplate (for now)
    fn get(&self) -> Param { self.0 }
    fn get_mut(&mut self) -> &mut Param { &mut self.0 }
}

impl RangedParam for Rotation {
    fn range(&self) -> (Param, Param) {
        (0.0, 360.0)
    }

    fn get(&self) -> Param { self.0 }
    fn get_mut(&mut self) -> &mut Param { &mut self.0 }
}

// implement ParamHolder for shape
impl ParamHolder for Shape {
    // 2 for dimensions + 1 for rotation
    fn param_count(&self) -> usize { self.dimensions.param_count() + 1 }

    fn get_param(&mut self, index: usize) -> &mut RangedParam {
        match index {
            0...1 => self.dimensions.get_param(index),
            2 => &mut self.rotation,
            _ => panic!("Bad index"),
        }
    }
}

// custom mutation generator to always mutate by the same amount
struct ConstGen(Param);

impl MutationGen for ConstGen {
    fn gen(&mut self) -> Param { self.0 }
}

// create - Rc and RefCell required for in-place mutation
let shape = Rc::new(RefCell::new(Shape {
    dimensions: ParamSet2d::new(Dimension(0.5), Dimension(0.5)),
    rotation: Rotation(0.0),
}));
println!("shape: {:?}", shape);

// mutate in place by adding 0.1 to all genes
mutate(shape.clone(), &mut ConstGen(0.1));
println!("mutated shape: {:?}", shape);
```
