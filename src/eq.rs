use super::*;

impl<T> PartialEq<f32> for FixedPointValue<T>
where
    T: Denomination {

}

impl<T> PartialEq<f64> for FixedPointValue<T>
where
    T: Denomination {

}
