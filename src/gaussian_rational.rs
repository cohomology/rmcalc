extern crate num;
extern crate num_rational;

use self::num::{BigInt, complex::Complex, rational::Ratio}; 
use self::num::Zero;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::convert::From;

#[derive(Debug, Clone)]
pub struct GaussianRational(Complex<Ratio<BigInt>>);

impl<'a> Add<&'a GaussianRational> for GaussianRational {
    type Output = GaussianRational; 
    fn add(self, other : &GaussianRational) -> GaussianRational {
        GaussianRational(self.0.add(&other.0))
    }
}

impl Add for GaussianRational {
    type Output = GaussianRational;
    fn add(self, other: GaussianRational) -> GaussianRational {
        GaussianRational(self.0.add(other.0))
    }
}

impl<'a> AddAssign<&'a GaussianRational> for GaussianRational {
    fn add_assign(&mut self, other : &GaussianRational) {
        self.0.add_assign(&other.0)
    }
} 

impl AddAssign for GaussianRational {
    fn add_assign(&mut self, other: GaussianRational) {
        self.0.add_assign(other.0)
    }
} 

impl<'a> Sub<&'a GaussianRational> for GaussianRational {
    type Output = GaussianRational; 
    fn sub(self, other : &GaussianRational) -> GaussianRational {
        GaussianRational(self.0.add(&other.0))
    }
} 

impl Sub for GaussianRational {
    type Output = GaussianRational;
    fn sub(self, other: GaussianRational) -> GaussianRational {
        GaussianRational(self.0.sub(other.0))
    }
}

impl<'a> SubAssign<&'a GaussianRational> for GaussianRational {
    fn sub_assign(&mut self, other : &GaussianRational) {
        self.0.sub_assign(&other.0)
    }
} 

impl SubAssign for GaussianRational {
    fn sub_assign(&mut self, other: GaussianRational) {
        self.0.sub_assign(other.0)
    }
} 

impl<'a> Mul<&'a GaussianRational> for GaussianRational {
    type Output = GaussianRational; 
    fn mul(self, other : &GaussianRational) -> GaussianRational {
        GaussianRational(self.0.mul(&other.0))
    }
} 

impl Mul for GaussianRational {
    type Output = GaussianRational;
    fn mul(self, other: GaussianRational) -> GaussianRational {
        GaussianRational(self.0.mul(other.0))
    }
}

impl<'a> MulAssign<&'a GaussianRational> for GaussianRational {
    fn mul_assign(&mut self, other : &GaussianRational) {
        self.0.mul_assign(&other.0)
    }
} 

impl MulAssign for GaussianRational {
    fn mul_assign(&mut self, other: GaussianRational) {
        self.0.mul_assign(other.0)
    }
} 

impl<'a> Div<&'a GaussianRational> for GaussianRational {
    type Output = GaussianRational; 
    fn div(self, other : &GaussianRational) -> GaussianRational {
        GaussianRational(self.0.div(&other.0))
    }
} 

impl Div for GaussianRational {
    type Output = GaussianRational;
    fn div(self, other: GaussianRational) -> GaussianRational {
        GaussianRational(self.0.div(other.0))
    }
}

impl<'a> DivAssign<&'a GaussianRational> for GaussianRational {
    fn div_assign(&mut self, other : &GaussianRational) {
        self.0.div_assign(&other.0)
    }
} 

impl DivAssign for GaussianRational {
    fn div_assign(&mut self, other: GaussianRational) {
        self.0.div_assign(other.0)
    }
} 

impl GaussianRational {
    pub fn new<T>(re_num : T, re_denom : T, im_num : T, im_denom : T) 
      -> GaussianRational
      where BigInt : From<T> {
        GaussianRational(Complex::new(
                Ratio::new(re_num.into(), re_denom.into()),
                Ratio::new(im_num.into(), im_denom.into()))) 
    }
    pub fn floor(&self) -> GaussianRational {
      GaussianRational(Complex::new(self.0.re.floor(), self.0.im.floor())) 
    }
    pub fn ceil(&self) -> GaussianRational {
      GaussianRational(Complex::new(self.0.re.ceil(), self.0.im.ceil())) 
    } 
    pub fn round(&self) -> GaussianRational {
      GaussianRational(Complex::new(self.0.re.round(), self.0.im.round())) 
    } 
    pub fn trunc(&self) -> GaussianRational {
      GaussianRational(Complex::new(self.0.re.trunc(), self.0.im.trunc())) 
    } 
    pub fn fract(&self) -> GaussianRational {
      GaussianRational(Complex::new(self.0.re.fract(), self.0.im.fract())) 
    } 
    pub fn is_zero(&self) -> bool {
        self.0.re.is_zero() && self.0.im.is_zero()
    }
    pub fn is_integer(&self) -> bool {
        self.0.re.is_integer() && self.0.im.is_zero()
    }
    pub fn is_pure_imaginary(&self) -> bool {
        self.0.re.is_zero()
    }
    pub fn norm_sqr(&self) -> GaussianRational {
        GaussianRational(Complex::new(self.0.norm_sqr(),
        Ratio::new(0.into(),1.into())))
    }
    pub fn inv(&self) -> GaussianRational {
        GaussianRational(self.0.inv())
    }
    // pub fn pow(&self, exp : i32) -> GaussianRational {
        
    // }
    // fn binExp(&mut self, exp : u32) {
    //     let mut copy = self.clone();
    //     self.set_to_one();
    //     while exp > 0 {
    //         if exp & 1 != 0 {
    //             self.mul_assign(&copy);
    //         }
    //         copy.mul_assign(copy);
    //         exp >>= 1;
    //     }
    // }
    // fn set_to_one(&mut self) {
    //     self.0.re = Ratio::new(1.into(), 1.into());
    //     self.0.im = Ratio::new(0.into(), 1.into());
    // }
} 
