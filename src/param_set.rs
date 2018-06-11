//! Helper structs to easily represent collections of related parameters.
/*! # Examples
  ```
# use genotype::{param_set::*, *};
// represents a length in space in a single dimension
struct Length(Param);

struct Cuboid(ParamSet3d<Length>);

impl RangedParam for Length {
    fn range(&self) -> (Param, Param) {
        (0.0, 10.0)
    }

    // ...
#
#    fn get(&self) -> Param {
#        self.0
#    }
#
#    fn get_mut(&mut self) -> &mut Param {
#        &mut self.0
#    }
}

// delegates to ParamSet
impl ParamHolder for Cuboid {
    fn param_count(&self) -> usize {
        self.0.param_count()
    }

    fn get_param(&mut self, index: usize) -> &mut RangedParam {
        match index {
            0...2 => self.0.get_param(index),
            _ => panic!("Bad index"),
        }
    }
}

# fn main() {
let cuboid = Cuboid(ParamSet3d::new(
    Length(0.25), Length(0.5), Length(0.75))
);

let (x, y, z) = cuboid.0.components_scaled();
assert!((x - 2.5).abs() < 0.00001);
assert!((y - 5.0).abs() < 0.00001);
assert!((z - 7.5).abs() < 0.00001);
# }
```

*/
use super::*;

/// Represents a collection of related parameters.
pub trait ParamSet<P: RangedParam>: ParamHolder {}

/// A 3D parameter set containing x, y and z fields.
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct ParamSet3d<P: RangedParam> {
    pub x: P,
    pub y: P,
    pub z: P,
}

/// A 2D parameter set containing x and y.
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct ParamSet2d<P: RangedParam> {
    pub x: P,
    pub y: P,
}

impl<P: RangedParam> ParamSet<P> for ParamSet3d<P> {}

impl<P: RangedParam> ParamSet<P> for ParamSet2d<P> {}

impl<P: RangedParam> ParamHolder for ParamSet3d<P> {
    fn param_count(&self) -> usize {
        3
    }

    fn get_param(&mut self, index: usize) -> &mut RangedParam {
        match index % 3 {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("out of bounds"),
        }
    }
}

impl<P: RangedParam> ParamHolder for ParamSet2d<P> {
    fn param_count(&self) -> usize {
        2
    }

    fn get_param(&mut self, index: usize) -> &mut RangedParam {
        match index % 2 {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("out of bounds"),
        }
    }
}

impl<P: RangedParam> ParamSet3d<P> {
    /// Creates a new parameter set with the given values.
    pub fn new(x: P, y: P, z: P) -> Self {
        Self { x, y, z }
    }

    /// Returns a tuple that contains each parameter, scaled with [get_scaled](../trait.RangedParam.html#method.get_scaled)
    pub fn components_scaled(&self) -> (Param, Param, Param) {
        (
            self.x.get_scaled(),
            self.y.get_scaled(),
            self.z.get_scaled(),
        )
    }
}

impl<P: RangedParam> ParamSet2d<P> {
    /// Creates a new parameter set with the given values.
    pub fn new(x: P, y: P) -> Self {
        Self { x, y }
    }

    /// Returns a tuple that contains each parameter, scaled with [get_scaled](../trait.RangedParam.html#method.get_scaled)
    pub fn components_scaled(&self) -> (Param, Param) {
        (self.x.get_scaled(), self.y.get_scaled())
    }
}
