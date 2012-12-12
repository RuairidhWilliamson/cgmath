use core::cmp::{Eq, Ord};

use std::cmp::FuzzyEq;

use funs::triganomic::{cos, sin};
use mat::{Mat3, Mat4};
use num::conv::cast;
use num::types::{Float, Number};
use quat::Quat;
use vec::Vec3;

/**
 * The base trait for angular units
 */
pub trait Angle<T>: Add<self,self>
                    Sub<self,self>
                    Mul<T,self>
                    Div<T,self>
                    // Div<self,T>      // TODO: not sure how to implement this, or if it is even possible...
                    Modulo<T,self>
                    // Modulo<self,T>   // TODO: not sure how to implement this, or if it is even possible...
                    Neg<self>
                    FuzzyEq
                    Eq Ord {
    static pure fn full_turn() -> self;
    static pure fn half_turn() -> self;
    static pure fn quadrant()  -> self;
    static pure fn sextant()   -> self;
    static pure fn octant()    -> self;
    static pure fn zero()      -> self;
    
    pure fn to_radians(&self) -> Radians<T>;
    pure fn to_degrees(&self) -> Degrees<T>;
    pure fn wrap(&self) -> self;
    pure fn opposite(&self) -> self;
}




// pub struct Radians<T>(T);    // error: internal compiler error: deref_cat() invoked on non-derefable type angle::Radians<'a>
pub enum Radians<T> = T;

pub impl<T:Copy Float> Radians<T>: Angle<T> {
    #[inline(always)] static pure fn full_turn()    -> Radians<T> { Radians(Float::two_pi())    }
    #[inline(always)] static pure fn half_turn()    -> Radians<T> { Radians(Float::pi())        }
    #[inline(always)] static pure fn quadrant()     -> Radians<T> { Radians(Float::frac_pi_2()) }
    #[inline(always)] static pure fn sextant()      -> Radians<T> { Radians(Float::frac_pi_3()) }
    #[inline(always)] static pure fn octant()       -> Radians<T> { Radians(Float::frac_pi_4()) }
    #[inline(always)] static pure fn zero()         -> Radians<T> { Radians(Number::zero())     }
    
    #[inline(always)] pure fn to_radians(&self) -> Radians<T> { *self }
    #[inline(always)] pure fn to_degrees(&self) -> Degrees<T> { Degrees(**self * cast(180.0 / Float::pi())) }
    
    #[inline(always)]
    pure fn wrap(&self) -> Radians<T> {
        let theta = (*self) % cast(2.0 * Float::pi());
        
        // keep in the domain of 0 to 1 rad
        if theta >= Angle::zero() {
            theta
        } else {
            theta + Angle::full_turn()
        }
    }
    
    #[inline(always)]
    pure fn opposite(&self) -> Radians<T> {
        (self + Angle::half_turn()).wrap()
    }
}
    
pub impl<T:Copy Float> Radians<T>: Add<Radians<T>, Radians<T>> {
    #[inline(always)]
    pure fn add(&self, rhs: &Radians<T>) -> Radians<T> {
        Radians(**self + **rhs)
    }
}
    
pub impl<T:Copy Float> Radians<T>: Sub<Radians<T>, Radians<T>> {
    #[inline(always)]
    pure fn sub(&self, rhs: &Radians<T>) -> Radians<T> {
        Radians(**self - **rhs)
    }
}
    
pub impl<T:Copy Float> Radians<T>: Mul<T, Radians<T>> {
    #[inline(always)]
    pure fn mul(&self, rhs: &T) -> Radians<T> {
        Radians(**self * *rhs)
    }
}
    
pub impl<T:Copy Float> Radians<T>: Div<T, Radians<T>> {
    #[inline(always)]
    pure fn div(&self, rhs: &T) -> Radians<T> {
        Radians(**self / *rhs)
    }
}
    
pub impl<T:Copy Float> Radians<T>: Modulo<T, Radians<T>> {
    #[inline(always)]
    pure fn modulo(&self, rhs: &T) -> Radians<T> {
        Radians(**self % *rhs)
    }
}
    
pub impl<T:Copy Float> Radians<T>: Neg<Radians<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Radians<T> {
        Radians(-**self)
    }
}

pub impl<T:Copy Float> Radians<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Radians<T>) -> bool {
        (*self).fuzzy_eq(&**other)
    }
}

pub impl<T:Copy Float> Radians<T>: Eq {
    #[inline(always)] pure fn eq(&self, other: &Radians<T>) -> bool { **self == **other }
    #[inline(always)] pure fn ne(&self, other: &Radians<T>) -> bool { **self != **other }
}

pub impl<T:Copy Float> Radians<T>: Ord {
    #[inline(always)] pure fn lt(&self, other: &Radians<T>) -> bool { **self <  **other }
    #[inline(always)] pure fn le(&self, other: &Radians<T>) -> bool { **self <= **other }
    #[inline(always)] pure fn ge(&self, other: &Radians<T>) -> bool { **self >= **other }
    #[inline(always)] pure fn gt(&self, other: &Radians<T>) -> bool { **self >  **other }
}

/**
 * # Example
 *
 * ~~~
 * assert fmt!("%s", Radians(1).to_str()) == ~"1 rad";
 * ~~~
 */
pub impl<T> Radians<T>: ToStr {
    pure fn to_str() -> ~str { fmt!("%? rad", *self) }
}





// pub struct Degrees<T>(T);    // error: internal compiler error: deref_cat() invoked on non-derefable type angle::Degrees<'a>
pub enum Degrees<T> = T;

pub impl<T:Copy Float> Degrees<T>: Angle<T> {
    #[inline(always)] static pure fn full_turn()    -> Degrees<T> { Degrees(cast(360.0)) }
    #[inline(always)] static pure fn half_turn()    -> Degrees<T> { Degrees(cast(180.0)) }
    #[inline(always)] static pure fn quadrant()     -> Degrees<T> { Degrees(cast(90.0))  }
    #[inline(always)] static pure fn sextant()      -> Degrees<T> { Degrees(cast(60.0))  }
    #[inline(always)] static pure fn octant()       -> Degrees<T> { Degrees(cast(45.0))  }
    #[inline(always)] static pure fn zero()         -> Degrees<T> { Degrees(cast(0.0))   }
    
    #[inline(always)] pure fn to_radians(&self) -> Radians<T> { Radians(**self * cast(Float::pi::<float>() / 180.0)) }
    #[inline(always)] pure fn to_degrees(&self) -> Degrees<T> { *self }
    
    #[inline(always)]
    pure fn wrap(&self) -> Degrees<T> {
        let theta = (*self) % cast(360);
        
        // keep in the domain of 0 to 360 degrees
        if theta >= Angle::zero() {
            theta
        } else {
            theta + Angle::full_turn()
        }
    }
    
    #[inline(always)]
    pure fn opposite(&self) -> Degrees<T> {
        (self + Angle::half_turn()).wrap()
    }
}

pub impl<T:Copy Float> Degrees<T>: Add<Degrees<T>, Degrees<T>> {
    #[inline(always)]
    pure fn add(&self, rhs: &Degrees<T>) -> Degrees<T> {
        Degrees(**self + **rhs)
    }
}
    
pub impl<T:Copy Float> Degrees<T>: Sub<Degrees<T>, Degrees<T>> {
    #[inline(always)]
    pure fn sub(&self, rhs: &Degrees<T>) -> Degrees<T> {
        Degrees(**self - **rhs)
    }
}
    
pub impl<T:Copy Float> Degrees<T>: Mul<T, Degrees<T>> {
    #[inline(always)]
    pure fn mul(&self, rhs: &T) -> Degrees<T> {
        Degrees(**self * *rhs)
    }
}
    
pub impl<T:Copy Float> Degrees<T>: Div<T, Degrees<T>> {
    #[inline(always)]
    pure fn div(&self, rhs: &T) -> Degrees<T> {
        Degrees(**self / *rhs)
    }
}
    
pub impl<T:Copy Float> Degrees<T>: Modulo<T, Degrees<T>> {
    #[inline(always)]
    pure fn modulo(&self, rhs: &T) -> Degrees<T> {
        Degrees(**self % *rhs)
    }
}
    
pub impl<T:Copy Float> Degrees<T>: Neg<Degrees<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Degrees<T> {
        Degrees(-**self)
    }
}

pub impl<T:Copy Float> Degrees<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Degrees<T>) -> bool {
        (*self).fuzzy_eq(&**other)
    }
}

pub impl<T:Copy Float> Degrees<T>: Eq {
    #[inline(always)] pure fn eq(&self, other: &Degrees<T>) -> bool { **self == **other }
    #[inline(always)] pure fn ne(&self, other: &Degrees<T>) -> bool { **self != **other }
}

pub impl<T:Copy Float> Degrees<T>: Ord {
    #[inline(always)] pure fn lt(&self, other: &Degrees<T>) -> bool { **self <  **other }
    #[inline(always)] pure fn le(&self, other: &Degrees<T>) -> bool { **self <= **other }
    #[inline(always)] pure fn ge(&self, other: &Degrees<T>) -> bool { **self >= **other }
    #[inline(always)] pure fn gt(&self, other: &Degrees<T>) -> bool { **self >  **other }
}

/**
 * # Example
 *
 * ~~~
 * assert fmt!("%s", Degrees(180.0).to_str()) == ~"180°";
 * ~~~
 */
pub impl<T> Degrees<T>: ToStr {
    pure fn to_str() -> ~str { fmt!("%?\xB0", *self) }
}