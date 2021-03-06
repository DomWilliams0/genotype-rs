//! This crate allows easy access and modification of the genotype of of an individual.
//! Independent of the phenotype, genes remain between 0 and 1, and can be indexed, iterated or modified in-place.
/*! # Examples
```
# use std::rc::Rc;
# use std::cell::RefCell;
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

*/

use std::ops::AddAssign;

#[cfg(feature = "serialize")]
extern crate serde;

#[macro_use]
#[cfg(feature = "serialize")]
extern crate serde_derive;

pub mod mutation;
pub mod param_set;

/// The type of a single gene.
pub type Param = f64; // TODO replace this with a generic parameter?

/** An entity with multiple parameters, i.e. a chromosone.
# Examples
```
# use genotype::*;
struct Weight(Param);
struct Height(Param);

impl RangedParam for Weight {
    fn range(&self) -> (Param, Param) {
        (40.0, 100.0)
    }
    // ...
#
#     fn get(&self) -> Param { self.0 }
#
#     fn get_mut(&mut self) -> &mut Param {&mut self.0}
}

impl RangedParam for Height {
    fn range(&self) -> (Param, Param) {
        (140.0, 185.0)
    }
    // ...
#
#     fn get(&self) -> Param { self.0 }
#
#     fn get_mut(&mut self) -> &mut Param {&mut self.0}
}


struct Human {
    weight: Weight,
    height: Height,
}

impl ParamHolder for Human {
    fn param_count(&self) -> usize {
        2
    }

    fn get_param(&mut self, index: usize) -> &mut RangedParam {
        match index {
            0 => &mut self.weight,
            1 => &mut self.height,
            _ => panic!("Bad index"),
        }
    }
}
```
*/
pub trait ParamHolder {
    /// The number of parameters/genes on this chromosone.
    fn param_count(&self) -> usize;

    /// Returns a mutable reference to the gene at the given index.
    /// # Panics
    /// If `index >= self.param_count()`
    fn get_param(&mut self, index: usize) -> &mut RangedParam;
}

/** Access to a gene's scaled value, i.e. the phenotype.

  The unscaled value is clamped between 0.0 and 1.0.
# Examples

```
# use genotype::*;
struct Weight(Param); // kg

impl RangedParam for Weight {

    fn range(&self) -> (Param, Param) {
        (40.0, 100.0) // arbitrary weight range
    }

    fn get(&self) -> Param { self.0 }

    fn get_mut(&mut self) -> &mut Param {&mut self.0}
}

# fn main() {
let mut weight = Weight(0.5);
assert_eq!(weight.get(), 0.5);
assert_eq!(weight.get_scaled(), 70.0);

// mutate the genotype a tad
*weight.get_mut() += 0.05;
assert_eq!(weight.get(), 0.55);
assert_eq!(weight.get_scaled(), 73.0);
# }
```
*/
pub trait RangedParam {
    /** The range of allowed values, in the form `(min, max).`
    # Examples
    The value in phenotype space remains between 0 and 1 (default implementation):
    ```
    # use genotype::Param;
    # struct X;
    # impl X {
    fn range(&self) -> (Param, Param) {
        (0.0, 1.0)
    }
    # }
    ```

    The phenotype value is between 100 and 200:
    ```
    # use genotype::Param;
    # struct X;
    # impl X {
    fn range(&self) -> (Param, Param) {
        (100.0, 200.0)
    }
    # }
    ```
    */
    fn range(&self) -> (Param, Param) {
        (0.0, 1.0) // unscaled
    }

    /// Returns the *unscaled* parameter value.
    fn get(&self) -> Param;

    /// Returns a mutable reference to the raw parameter value.
    fn get_mut(&mut self) -> &mut Param;

    /// Returns the parameter value scaled to the range returned by [range](#method.range) i.e. the gene expressed in the phenotype.
    fn get_scaled(&self) -> Param {
        let (min, max) = self.range();
        (max - min) * self.get() + min
    }
}

impl<'a> AddAssign<Param> for &'a mut RangedParam {
    fn add_assign(&mut self, rhs: Param) {
        let clamped = {
            let val = *self.get_mut() + rhs;
            if val < 0.0 {
                0.0
            } else if val > 1.0 {
                1.0
            } else {
                val
            }
        };
        *self.get_mut() = clamped;
    }
}

#[cfg(test)]
macro_rules! assert_feq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        let diff = (a - b).abs();
        assert!(diff < 0.00001, "{} !~= {}", a, b);
    }};
}

#[cfg(test)]
mod tests {
    use super::{mutation::*, param_set::*, *};
    use std::cell::RefCell;
    use std::rc::Rc;

    struct TestParam(Param);

    struct TestHolder {
        x: TestParam,
    }

    impl ParamHolder for TestHolder {
        fn param_count(&self) -> usize {
            1
        }

        fn get_param(&mut self, index: usize) -> &mut RangedParam {
            match index {
                0 => &mut self.x,
                _ => panic!("Bad param index"),
            }
        }
    }

    impl RangedParam for TestParam {
        fn range(&self) -> (Param, Param) {
            (0.0, 20.0)
        }

        fn get(&self) -> Param {
            self.0
        }

        fn get_mut(&mut self) -> &mut Param {
            &mut self.0
        }
    }

    struct ConstGen(Param);

    impl MutationGen for ConstGen {
        fn gen(&mut self) -> Param {
            self.0
        }
    }

    #[test]
    fn test_mutate() {
        let holder = Rc::new(RefCell::new(TestHolder {
            x: TestParam { 0: 0.0 },
        }));
        mutate(holder.clone(), &mut ConstGen { 0: 0.5 });

        assert_feq!(holder.borrow().x.get_scaled(), 10.0);
    }

    #[test]
    fn test_clamp() {
        let holder = Rc::new(RefCell::new(TestHolder { x: TestParam(0.0) }));
        mutate(holder.clone(), &mut ConstGen { 0: -0.5 });
        assert_feq!(holder.borrow().x.get_scaled(), 0.0);

        // should be equal to max
        mutate(holder.clone(), &mut ConstGen { 0: 1.5 });
        assert_feq!(holder.borrow().x.get_scaled(), 20.0);
    }

    #[derive(Debug)]
    struct Pos(Param);

    #[derive(Debug)]
    struct MultiShape(ParamSet3d<Pos>);

    impl RangedParam for Pos {
        fn range(&self) -> (Param, Param) {
            (0.0, 10.0)
        }

        fn get(&self) -> Param {
            self.0
        }

        fn get_mut(&mut self) -> &mut Param {
            &mut self.0
        }
    }

    impl ParamHolder for MultiShape {
        fn param_count(&self) -> usize {
            3
        }

        fn get_param(&mut self, index: usize) -> &mut RangedParam {
            match index {
                0...2 => self.0.get_param(index),
                _ => panic!("Bad param index"),
            }
        }
    }

    #[test]
    fn test_paramset() {
        let holder = Rc::new(RefCell::new(MultiShape(ParamSet3d::new(
            Pos(0.1),
            Pos(0.1),
            Pos(0.1),
        ))));
        mutate(holder.clone(), &mut ConstGen { 0: 0.15 });

        let expected = 2.5; // 10.0 * 0.25
        let pos = &holder.borrow().0;
        assert_feq!(pos.x.get_scaled(), expected);
        assert_feq!(pos.y.get_scaled(), expected);
        assert_feq!(pos.z.get_scaled(), expected);
    }
}
