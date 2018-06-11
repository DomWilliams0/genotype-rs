/*! Gene mutation operators.

# Examples
```
# use genotype::*;
# use mutation::*;
struct TotallyRandomGen;

impl MutationGen for TotallyRandomGen {
    fn gen(&mut self) -> Param {
        0.4
    }
}

// as seen before
struct Weight(Param);

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

struct Human {
    weight: Weight,
}

impl ParamHolder for Human {
    // ...
#    fn param_count(&self) -> usize {
#        1
#    }
#
#    fn get_param(&mut self, index: usize) -> &mut RangedParam {
#        match index {
#            0 => &mut self.weight,
#            _ => panic!("Bad index"),
#        }
#    }
}
# fn main() {
# use std::cell::RefCell;
# use std::rc::Rc;

let human = Rc::new(RefCell::new(
    Human { weight: Weight(0.1) }
));
// unscaled value is initially 0.1
assert!((human.borrow().weight.get_scaled() - 46.0) < 0.00001);

let mut gen = TotallyRandomGen{};
mutate(human.clone(), &mut gen);

// unscaled value has mutated to 0.5
assert!((human.borrow().weight.get_scaled() - 70.0) < 0.00001);
# }
```
*/

use super::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Produces values to add to an unscaled gene value.
///
/// Keep in mind that the unscaled value is clamped between 0.0 and 1.0.
pub trait MutationGen {
    /// Returns a value that is added to an unscaled gene.
    fn gen(&mut self) -> Param;
}

/// Mutates the given `ParamHolder` with the given `MutationGen` by iterating through all genes and
/// adding to each the result of calling the mutation generator.
///
/// See [examples](index.html#example).
pub fn mutate<P: ParamHolder, MG: MutationGen>(param_holder: Rc<RefCell<P>>, mut_gen: &mut MG) {
    let n = param_holder.borrow().param_count();

    for i in 0..n {
        let mut holder = param_holder.borrow_mut();
        let mut p: &mut RangedParam = holder.get_param(i);
        p += mut_gen.gen();
    }
}
