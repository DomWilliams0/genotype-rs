use super::*;

/// Helper struct to represent a collection of related parameters in multiple dimensions.
pub trait ParamSet<P: RangedParam>: ParamHolder {}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct ParamSet3d<P: RangedParam> {
    pub x: P,
    pub y: P,
    pub z: P,
}

impl<P: RangedParam> ParamSet<P> for ParamSet3d<P> {}

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

impl<P: RangedParam> ParamSet3d<P> {
    pub fn new(x: P, y: P, z: P) -> Self {
        Self { x, y, z }
    }

    pub fn components_scaled(&self) -> (Param, Param, Param) {
        (
            self.x.get_scaled(),
            self.y.get_scaled(),
            self.z.get_scaled(),
        )
    }
}
