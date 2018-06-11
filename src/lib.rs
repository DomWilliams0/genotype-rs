//! This crate allows easy access and modification of the genotype of of an individual.
//! Independent of the phenotype, genes remain between 0 and 1, and can be indexed, iterated or modified in-place.
//! TODO: full example

use std::cell::RefCell;
use std::ops::AddAssign;
use std::rc::Rc;

#[cfg(feature = "serialize")]
extern crate serde;

#[macro_use]
#[cfg(feature = "serialize")]
extern crate serde_derive;

pub mod param_set;

/// The type of a single gene.
pub type Param = f64; // TODO replace this with a generic parameter?

/** An entity with multiple parameters, i.e. a chromosone.
# Examples
```
# use genotype::*;
struct Weight(f64);
struct Height(f64);

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

/** Access to a decoded gene's value, i.e. the phenotype.
# Examples

```
# use genotype::*;
struct Weight(f64); // kg

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

/// A mutation generator, that produces an offset to add to the current value.
/// Should range between -1.0 and 1.0, but the result will be clamped anyway
pub trait MutationGen {
    fn gen(&mut self) -> Param;
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

pub fn mutate<P: ParamHolder, MG: MutationGen>(param_holder: Rc<RefCell<P>>, mut_gen: &mut MG) {
    let n = param_holder.borrow().param_count();

    for i in 0..n {
        let mut holder = param_holder.borrow_mut();
        let mut p: &mut RangedParam = holder.get_param(i);
        p += mut_gen.gen();
    }
}

#[cfg(test)]
mod tests {
    use super::{param_set::*, *};

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

        let expected = 10.0; // 20.0 * 0.5
        let diff = (holder.borrow().x.get_scaled() - expected).abs();
        assert!(diff < 0.001);
    }

    #[test]
    fn test_clamp() {
        let holder = Rc::new(RefCell::new(TestHolder {
            x: TestParam { 0: 0.0 },
        }));
        mutate(holder.clone(), &mut ConstGen { 0: -0.5 });
        assert!(holder.borrow().x.get_scaled() < 0.001);

        // should be equal to max
        mutate(holder.clone(), &mut ConstGen { 0: 1.5 });
        assert!((holder.borrow().x.get_scaled() - 20.0).abs() < 0.001);
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
        assert!((pos.x.get_scaled() - expected).abs() < 0.001);
        assert!((pos.y.get_scaled() - expected).abs() < 0.001);
        assert!((pos.z.get_scaled() - expected).abs() < 0.001);
    }
}
