//Author: Everett Sullivan.
//Date created: March 14th 2019
//Purpose: Contains a rational struct for doing computations on rationals
//TO DO: Division with usize and isize
//       Reduce reliance on function calls to increase efficiency

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;
use std::cmp::Ordering;
use std::cmp;
use std::fmt;

#[derive(Debug)]
pub struct Rational {
    sign: bool, //is true if a negative number, false if zero or positive
    numerator: usize, //numerator
    denominator: usize, //denominator
}
//in all operations with rationals, rationals are immedately reduced to lowest form
//This way any two rational structs that represent the same number will be equal.
//If the numerator is 0, we will have the denominator be 1
impl Rational {
    pub fn new_signed_rational(numerator: usize, denominator: usize, sign: bool) -> Rational{
        if denominator == 0 {
            panic!("Attempted creation of a rational with zero denominator!");
        }
        let my_gcd;
        let mut my_sign = sign;
        let mut my_denominator = denominator;
        let mut my_numerator = numerator;
        if my_numerator == 0 {
            my_sign = false;
            my_denominator = 1;
        }else{
            my_gcd = gcd(numerator,my_denominator);
            my_denominator = my_denominator/my_gcd;
            my_numerator = my_numerator/my_gcd;
        }
        Rational{
            sign: my_sign,
            numerator: my_numerator,
            denominator: my_denominator,
        }
    }

    //if no sign is not given assume it is a non-negative ratinal
    pub fn new_rational(numerator: usize, denominator: usize) -> Rational{
        if denominator == 0 {
            panic!("Attempted creation of a rational with zero denominator!");
        }
        let my_gcd;
        let mut my_denominator = denominator;
        let mut my_numerator = numerator;
        if my_numerator == 0 {
            my_denominator = 1;
        }else{
            my_gcd = gcd(numerator,my_denominator);
            my_denominator = my_denominator/my_gcd;
            my_numerator = my_numerator/my_gcd;
        }
        Rational{
            sign: false,
            numerator: my_numerator,
            denominator: my_denominator,
        }
    }

    pub fn new_rational_from_integers(numerator: isize, denominator: isize) -> Rational{
        if denominator == 0 {
            panic!("Attempted creation of a rational with zero denominator!");
        }
        let my_gcd;
        let my_sign;
        let mut my_denominator:usize;
        let mut my_numerator:usize;
        let neg_numerator:bool;
        let neg_denominator:bool;
        if numerator == 0 {
            my_sign = false;
            my_numerator = 0;
            my_denominator = 1;
        }else{
            if numerator < 0 {
                neg_numerator = true;
                my_numerator = (-numerator) as usize;
            }else{
                neg_numerator = false;
                my_numerator = numerator as usize;
            }
            if denominator < 0{
                neg_denominator = true;
                my_denominator = (-denominator) as usize;
            }else{
                neg_denominator = false;
                my_denominator = denominator as usize;
            }

            if (neg_numerator & !neg_denominator) | (!neg_numerator & neg_denominator) {
                my_sign = true;
            }else{
                my_sign = false;
            }
            my_gcd = gcd(my_numerator,my_denominator);
            my_denominator = my_denominator/my_gcd;
            my_numerator = my_numerator/my_gcd;
        }
        Rational{
            sign: my_sign,
            numerator: my_numerator,
            denominator: my_denominator,
        }
    }

    pub fn new_rational_from_unsigned_integers(numerator: usize, denominator: usize) -> Rational{
        if denominator == 0 {
            panic!("Attempted creation of a rational with zero denominator!");
        }
        let my_gcd;
        let my_denominator:usize;
        let my_numerator:usize;
        if numerator == 0 {
            my_numerator = 0;
            my_denominator = 1;
        }else{
            my_gcd = gcd(numerator,denominator);
            my_denominator = denominator/my_gcd;
            my_numerator = numerator/my_gcd;
        }
        Rational{
            sign: false,
            numerator: my_numerator,
            denominator: my_denominator,
        }
    }

    pub fn new_rational_from_integer(integer: isize) -> Rational{
        let my_sign;
        let my_numerator;
        if integer < 0 {
            my_sign = true;
            my_numerator = (-integer) as usize;
        }else{
            my_sign = false;
            my_numerator =  integer as usize;
        }
        Rational{
            sign: my_sign,
            numerator: my_numerator,
            denominator: 1,
        }
    }

    pub fn new_rational_from_unsigned_integer(integer: usize) -> Rational{
        Rational{
            sign: false,
            numerator: integer,
            denominator: 1,
        }
    }


    //getters for Rational struct fields
    pub fn denominator(&self) -> usize{
	    self.denominator
	}

    pub fn numerator(&self) -> usize{
	    self.numerator
	}

    pub fn is_neg(&self) -> bool{
	    self.sign
	}

    //floor
    //Purpose:
    //    Returns the largest integer less than or equal to the current rational.
    //Pre-conditions:
    //    None.
    pub fn floor(&self) -> isize{
	    if self.denominator == 1 {
            if self.sign {
                -(self.numerator as isize)
            }else{
                self.numerator as isize
            }
        }else{
            if self.sign {
                -((self.numerator as isize) / (self.denominator as isize)) - 1
            }else{
                (self.numerator / self.denominator) as isize
            }
        }
	}

    //ceil
    //Purpose:
    //    Returns the samllest integer greater than or equal to the current rational.
    //Pre-conditions:
    //    None.
    pub fn ceil(&self) -> isize{
        if self.denominator == 1 {
            if self.sign {
                -(self.numerator as isize)
            }else{
                self.numerator as isize
            }
        }else{
            if self.sign {
                -((self.numerator as isize) / (self.denominator as isize))
            }else{
                (self.numerator / self.denominator + 1) as isize
            }
        }
	}

    //rational_floor
    //Purpose:
    //    Returns the largest rational number less than or equal to the current rational which has the given denominator.
    //Pre-conditions:
    //    denom must not be zero. (If zero is used, return the zero rational)
    pub fn rational_floor(&self,denom: usize) -> Rational{
        let a = Rational::new_rational_from_unsigned_integer(denom);
	    Rational::new_rational_from_integers((a*(*self)).floor() as isize,denom as isize)
	}

    //rational_ceil
    //Purpose:
    //    Returns the smallest rational number greater than or equal to the current rational which has the given denominator.
    //Pre-conditions:
    //    denom must not be zero. (If zero is used, return the zero rational)
    pub fn rational_ceil(&self,denom: usize) -> Rational{
        let a = Rational::new_rational_from_unsigned_integer(denom);
	    Rational::new_rational_from_integers((a*(*self)).ceil() as isize,denom as isize)
	}

    //abs
    //Purpose:
    //    Returns the absolute value of the given rational.
    //Pre-conditions:
    //    None
    pub fn abs(&self) -> Rational{
        let a = Rational::new_rational_from_unsigned_integers(self.numerator,self.denominator);
        a
	}
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match (self.sign, self.denominator){
            (true,1) => {
                write!(f, "-{}", self.numerator)
            }
            (true,_) => {
                write!(f, "-{}/{}", self.numerator, self.denominator)
            }
            (false,1) => {
                write!(f, "{}", self.numerator)
            }
            (false,_) => {
                write!(f, "{}/{}", self.numerator, self.denominator)
            }
        }
    }
}

impl Copy for Rational {}

impl Clone for Rational {
    fn clone(&self) -> Rational {
        *self
    }
}

impl Eq for Rational {}// since we always reduce a rational to lowest form, it is sufficent to check that the fields are exactly the same

impl PartialEq for Rational { // since we always reduce a rational to lowest form, it is sufficent to test that the fields are exactly the same
    fn eq(&self, rhs: &Rational) -> bool {
        (self.sign == rhs.sign) & (self.numerator == rhs.numerator) & (self.denominator == rhs.denominator)
    }
}

impl Ord for Rational {
    fn cmp(&self, rhs: &Rational) -> Ordering {
        if self.sign & !rhs.sign {
            Ordering::Less
        }else if !self.sign & rhs.sign{
            Ordering::Greater
        }else if self.sign & rhs.sign{
            match (self.numerator*rhs.denominator).cmp(&(self.denominator*rhs.numerator)){
                Ordering::Equal => Ordering::Equal,
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
            }
        }else{
            (self.numerator*rhs.denominator).cmp(&(self.denominator*rhs.numerator))
        }
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, rhs: &Rational) -> Option<Ordering> {
        if self.sign & !rhs.sign {
            Some(Ordering::Less)
        }else if !self.sign & rhs.sign{
            Some(Ordering::Greater)
        }else if self.sign & rhs.sign{
            match (self.numerator*rhs.denominator).cmp(&(self.denominator*rhs.numerator)){
                Ordering::Equal => Some(Ordering::Equal),
                Ordering::Less => Some(Ordering::Greater),
                Ordering::Greater => Some(Ordering::Less),
            }
        }else{
            Some((self.numerator*rhs.denominator).cmp(&(self.denominator*rhs.numerator)))
        }
    }
}

impl Add<Rational> for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Rational {
        if self.numerator == 0 {
            return rhs;
        }else if  rhs.numerator == 0 {
            return self;
        } else {
            match (self.sign, rhs.sign){
                (true,true) => {
                    return Rational::new_signed_rational(self.denominator*rhs.numerator + self.numerator*rhs.denominator,self.denominator*rhs.denominator,true);
                }
                (false,false) => {
                    return Rational::new_rational(self.denominator*rhs.numerator + self.numerator*rhs.denominator,self.denominator*rhs.denominator);
                }
                (false,true) => {
                    return Rational::new_rational_from_integers(((self.numerator*rhs.denominator) as isize) - ((self.denominator*rhs.numerator) as isize),(self.denominator*rhs.denominator) as isize);
                }
                (true,false) => {
                    return Rational::new_rational_from_integers(((self.denominator*rhs.numerator) as isize) - ((self.numerator*rhs.denominator) as isize),(self.denominator*rhs.denominator) as isize);
                }
            }
        }
    }
}

impl Add<usize> for Rational {
    type Output = Rational;

    fn add(self, rhs: usize) -> Rational {
        self + Rational::new_rational_from_unsigned_integer(rhs)
    }
}

impl Add<isize> for Rational {
    type Output = Rational;

    fn add(self, rhs: isize) -> Rational {
        self + Rational::new_rational_from_integer(rhs)
    }
}

impl Sub<Rational> for Rational {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Rational {
        self + (-rhs)
    }
}

impl Sub<usize> for Rational {
    type Output = Rational;

    fn sub(self, rhs: usize) -> Rational {
        self + (-(rhs as isize))
    }
}

impl Sub<isize> for Rational {
    type Output = Rational;

    fn sub(self, rhs: isize) -> Rational {
        self + (-rhs)
    }
}

impl Mul<Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Rational {
        let my_first_gcd;
        let my_second_gcd;
        let mut my_sign = false;
        let my_denominator;
        let my_numerator;
        if (self.numerator == 0) || (rhs.numerator == 0) {
            my_sign = false;
            my_denominator = 1;
            my_numerator = 0;
        }else {
            my_first_gcd = gcd(self.numerator,rhs.denominator);
            my_second_gcd = gcd(rhs.numerator,self.denominator);
            if (self.sign & !rhs.sign) | (!self.sign & rhs.sign) {
                my_sign = true;
            }
            my_denominator = (self.denominator/my_second_gcd)*(rhs.denominator/my_first_gcd);
            my_numerator = (self.numerator/my_first_gcd)*(rhs.numerator/my_second_gcd);
        }
        Rational{
            sign: my_sign,
            numerator: my_numerator,
            denominator: my_denominator,
        }
    }
}

impl Mul<usize> for Rational {
    type Output = Rational;

    fn mul(self, rhs: usize) -> Rational {
        let my_gcd;
        let my_sign;
        let my_denominator;
        let my_numerator;
        if (rhs == 0) || (self.numerator == 0) {
            my_sign = false;
            my_denominator = 1;
            my_numerator = 0;
        }else {
            my_gcd = gcd(self.denominator,rhs);
            my_sign = self.sign;
            my_denominator = self.denominator/my_gcd;
            my_numerator = self.numerator*(rhs/my_gcd);
        }
        Rational{
            sign: my_sign,
            numerator: my_numerator,
            denominator: my_denominator,
        }
    }
}

impl Mul<isize> for Rational {
    type Output = Rational;

    fn mul(self, rhs: isize) -> Rational {
        let my_gcd;
        let my_sign;
        let my_denominator;
        let my_numerator;
        let my_rhs;
        if (rhs == 0) || (self.numerator == 0) {
            my_sign = false;
            my_denominator = 1;
            my_numerator = 0;
        }else {
            if rhs < 0 {
                my_sign = !self.sign;
                my_rhs = (-rhs) as usize;
            }else{
                my_rhs = rhs as usize;
                my_sign = self.sign;
            }
            my_gcd = gcd(self.denominator,my_rhs);
            my_denominator = self.denominator/my_gcd;
            my_numerator = self.numerator*(my_rhs/my_gcd);
        }
        Rational{
            sign: my_sign,
            numerator: my_numerator,
            denominator: my_denominator,
        }
    }
}

impl Div<Rational> for Rational {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Rational {
        let my_first_gcd;
        let my_second_gcd;
        let mut my_sign = false;
        let my_denominator;
        let my_numerator;
        if rhs.numerator  == 0 {
            panic!("Attempted division by zero (Rationals)!");
        } else if self.numerator == 0 {
            my_sign = false;
            my_denominator = 1;
            my_numerator = 0;
        } else {
            my_first_gcd = gcd(self.numerator,rhs.numerator);
            my_second_gcd = gcd(rhs.denominator,self.denominator);
            if (self.sign & !rhs.sign) | (!self.sign & rhs.sign) {
                my_sign = true;
            }
            my_denominator = (self.denominator/my_second_gcd)*(rhs.numerator/my_first_gcd);
            my_numerator = (self.numerator/my_first_gcd)*(rhs.denominator/my_second_gcd);
        }
        Rational{
            sign: my_sign,
            numerator: my_numerator,
            denominator: my_denominator,
        }
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        let mut my_sign = self.sign;
        if self.numerator != 0 { //zero always has sign being false so don't switch it in that case
            my_sign = !my_sign;
        }
        Rational{
            sign: my_sign,
            numerator: self.numerator,
            denominator: self.denominator,
        }
    }
}

//gcd
//Purpose:
//    Given two positive numbers returns their gcd.
//Pre-conditions:
//    Both numbers x and y are positive.
pub fn gcd(x: usize, y: usize) -> usize{
    let mut first_num = x;
	let mut second_num = y;
	let mut remainder;
	remainder = first_num % second_num; //since x and y are not zero, don't  need to worry about % 0.
	while remainder != 0{
	    first_num = second_num;
        second_num = remainder;
		remainder = first_num % second_num;
	}
	second_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Attempted creation of a rational with zero denominator!")]
    fn reject_zero_denominator() {
        Rational::new_rational(1,0);
    }

    #[test]
    fn equal_rationals() {
        assert_eq!(Rational::new_rational(1,2),Rational::new_rational(2,4));
        assert_eq!(Rational::new_rational(2,5),Rational::new_rational(20,50));
        assert_eq!(Rational::new_rational(0,2),Rational::new_rational(0,3));
        assert_eq!(-Rational::new_rational(1,2),Rational::new_signed_rational(2,4,true));
        assert_eq!(Rational::new_rational(1,2),Rational::new_signed_rational(2,4,false));
        assert_eq!(Rational::new_signed_rational(0,1,false),Rational::new_signed_rational(0,1,true));
        assert_eq!(Rational::new_rational(1,2),Rational::new_rational_from_integers(-1,-2));
        assert_eq!(Rational::new_rational(0,17),Rational::new_rational_from_integers(0,5));
        assert_eq!(Rational::new_rational(0,17),Rational::new_rational_from_integers(0,-2));
        assert_eq!(-Rational::new_rational(1,3),Rational::new_rational_from_integers(-1,3));
        assert_eq!(Rational::new_rational_from_integers(1,-2),Rational::new_rational_from_integers(-1,2));
        assert_eq!(Rational::new_rational_from_integers(1,-2),Rational::new_rational_from_integers(-4,8));
        assert_eq!(Rational::new_rational_from_integers(2,3),Rational::new_rational_from_integers(-4,-6));
        assert_eq!(Rational::new_rational_from_unsigned_integers(1,2),Rational::new_rational_from_integers(4,8));
        assert_eq!(Rational::new_rational_from_unsigned_integer(0),Rational::new_rational_from_integers(0,1));
        assert_eq!(Rational::new_rational_from_unsigned_integer(7),Rational::new_rational_from_integers(14,2));
        assert_eq!(Rational::new_rational_from_integer(-1),Rational::new_rational_from_integers(-1,1));
        assert_eq!(Rational::new_rational_from_integer(-10),Rational::new_rational_from_integers(-30,3));
    }

    #[test]
    fn floor_and_ceiling(){
        assert_eq!(Rational::new_rational(0,1).floor(),0);
        assert_eq!(Rational::new_rational(0,1).ceil(),0);
        assert_eq!(Rational::new_rational(1,1).floor(),1);
        assert_eq!(Rational::new_rational(1,1).ceil(),1);
        assert_eq!(Rational::new_signed_rational(1,1,true).floor(),-1);
        assert_eq!(Rational::new_signed_rational(1,1,true).ceil(),-1);
        assert_eq!(Rational::new_rational(1,2).floor(),0);
        assert_eq!(Rational::new_rational(1,2).ceil(),1);
        assert_eq!(Rational::new_signed_rational(1,2,true).floor(),-1);
        assert_eq!(Rational::new_signed_rational(1,2,true).ceil(),0);
        assert_eq!(Rational::new_rational(16,5).floor(),3);
        assert_eq!(Rational::new_rational(71,12).ceil(),6);
    }

    #[test]
    fn floor_and_ceiling_rational(){
        assert_eq!(Rational::new_rational(1,2).rational_floor(2),Rational::new_rational(1,2));
        assert_eq!(Rational::new_rational(3,4).rational_floor(2),Rational::new_rational(1,2));
        assert_eq!(Rational::new_signed_rational(1,4,true).rational_floor(2),Rational::new_signed_rational(1,2,true));
        assert_eq!(Rational::new_rational(1,2).rational_ceil(2),Rational::new_rational(1,2));
        assert_eq!(Rational::new_rational(3,4).rational_ceil(2),Rational::new_rational(1,1));
        assert_eq!(Rational::new_signed_rational(1,4,true).rational_ceil(2),Rational::new_signed_rational(0,1,false));
    }

    #[test]
    fn compare_rationals() {
        assert!(Rational::new_rational(1,3) < Rational::new_rational(1,2));
        assert!(-Rational::new_rational(1,3) < Rational::new_rational(1,2));
        assert!(Rational::new_rational(1,3) > -Rational::new_rational(1,2));
        assert!(Rational::new_rational(1,3) <= Rational::new_rational(1,3));
        assert!(!(Rational::new_rational(1,3) < Rational::new_rational(1,3)));
    }

    #[test]
    fn add_rational_to_rational(){
        let two = Rational::new_rational(2,1);
        let one = Rational::new_rational(1,1);
        let one_half = Rational::new_rational(1,2);
        let one_third = Rational::new_rational(1,3);
        let one_fourth = Rational::new_rational(1,4);
        let neg_two = Rational::new_rational_from_integers(-2,1);
        let neg_one = Rational::new_rational_from_integers(-1,1);
        let neg_one_half = Rational::new_rational_from_integers(-1,2);
        let neg_one_third = Rational::new_rational_from_integers(-1,3);
        let neg_one_fourth = Rational::new_rational_from_integers(-1,4);
        let neg_one_sixth = Rational::new_rational_from_integers(-1,6);
        let one_sixth = Rational::new_rational_from_integers(1,6);
        let zero = Rational::new_rational_from_integers(0,1);
        assert_eq!(zero + zero,zero);
        assert_eq!(zero + one,one);
        assert_eq!(one + zero,one);
        assert_eq!(zero + neg_one,neg_one);
        assert_eq!(neg_one + zero,neg_one);
        assert_eq!(one + neg_one,zero);
        assert_eq!(neg_one + one,zero);
        assert_eq!(one + one,two);
        assert_eq!(neg_one + neg_one,neg_two);
        assert_eq!(one_half + neg_one_half,zero);
        assert_eq!(neg_one_half + one_half,zero);
        assert_eq!(one_fourth + neg_one_half,neg_one_fourth);
        assert_eq!(neg_one_half + one_fourth,neg_one_fourth);
        assert_eq!(one_third + neg_one_third,zero);
        assert_eq!(neg_one_third + one_sixth,neg_one_sixth);
    }

    #[test]
    fn add_isize_to_rational(){
        let two = Rational::new_rational(2,1);
        let three_halves = Rational::new_rational(3,2);
        let one = Rational::new_rational(1,1);
        let one_half = Rational::new_rational(1,2);
        let one_third = Rational::new_rational(1,3);
        let zero = Rational::new_rational_from_integers(0,1);
        let neg_one_half = Rational::new_rational_from_integers(-1,2);
        let neg_two_thirds = Rational::new_rational_from_integers(-2,3);
        let neg_one = Rational::new_rational_from_integers(-1,1);
        let neg_two = Rational::new_rational_from_integers(-2,1);
        assert_eq!(zero + (0 as isize),zero);
        assert_eq!(zero + (1 as isize),one);
        assert_eq!(zero + (-1 as isize),neg_one);
        assert_eq!(one + (0 as isize),one);
        assert_eq!(one + (1 as isize),two);
        assert_eq!(one + (-1 as isize),zero);
        assert_eq!(neg_one + (0 as isize),neg_one);
        assert_eq!(neg_one + (1 as isize),zero);
        assert_eq!(neg_one + (-1 as isize),neg_two);
        assert_eq!(neg_one_half + (1 as isize),one_half);
        assert_eq!(neg_one_half + (2 as isize),three_halves);
        assert_eq!(neg_one_half + (0 as isize),neg_one_half);
        assert_eq!(one_half + (0 as isize),one_half);
        assert_eq!(one_half + (1 as isize),three_halves);
        assert_eq!(one_half + (-1 as isize),neg_one_half);
        assert_eq!(three_halves + (-2 as isize),neg_one_half);
        assert_eq!(one_third + (-1 as isize),neg_two_thirds);
        assert_eq!(neg_two_thirds + (1 as isize),one_third);
    }

    #[test]
    fn add_usize_to_rational(){
        let two = Rational::new_rational(2,1);
        let three_halves = Rational::new_rational(3,2);
        let one = Rational::new_rational(1,1);
        let one_half = Rational::new_rational(1,2);
        let one_third = Rational::new_rational(1,3);
        let zero = Rational::new_rational_from_integers(0,1);
        let neg_one_half = Rational::new_rational_from_integers(-1,2);
        let neg_two_thirds = Rational::new_rational_from_integers(-2,3);
        let neg_one = Rational::new_rational_from_integers(-1,1);
        assert_eq!(zero + (0 as usize),zero);
        assert_eq!(zero + (1 as usize),one);
        assert_eq!(one + (0 as usize),one);
        assert_eq!(one + (1 as usize),two);
        assert_eq!(neg_one + (0 as usize),neg_one);
        assert_eq!(neg_one + (1 as usize),zero);
        assert_eq!(neg_one_half + (1 as usize),one_half);
        assert_eq!(neg_one_half + (2 as usize),three_halves);
        assert_eq!(neg_one_half + (0 as usize),neg_one_half);
        assert_eq!(one_half + (0 as usize),one_half);
        assert_eq!(one_half + (1 as usize),three_halves);
        assert_eq!(neg_two_thirds + (1 as usize),one_third);
    }

    #[test]
    fn sub_rational_from_rational(){
        let two = Rational::new_rational(2,1);
        let one = Rational::new_rational(1,1);
        let one_half = Rational::new_rational(1,2);
        let one_third = Rational::new_rational(1,3);
        let one_fourth = Rational::new_rational(1,4);
        let neg_two = Rational::new_rational_from_integers(-2,1);
        let neg_one = Rational::new_rational_from_integers(-1,1);
        let neg_one_half = Rational::new_rational_from_integers(-1,2);
        let neg_one_third = Rational::new_rational_from_integers(-1,3);
        let neg_one_fourth = Rational::new_rational_from_integers(-1,4);
        let neg_one_sixth = Rational::new_rational_from_integers(-1,6);
        let one_sixth = Rational::new_rational_from_integers(1,6);
        let zero = Rational::new_rational_from_integers(0,1);
        assert_eq!(zero + zero,zero);
        assert_eq!(zero + one,one);
        assert_eq!(one + zero,one);
        assert_eq!(zero + neg_one,neg_one);
        assert_eq!(neg_one + zero,neg_one);
        assert_eq!(one + neg_one,zero);
        assert_eq!(neg_one + one,zero);
        assert_eq!(one + one,two);
        assert_eq!(neg_one + neg_one,neg_two);
        assert_eq!(one_half + neg_one_half,zero);
        assert_eq!(neg_one_half + one_half,zero);
        assert_eq!(one_fourth + neg_one_half,neg_one_fourth);
        assert_eq!(neg_one_half + one_fourth,neg_one_fourth);
        assert_eq!(one_third + neg_one_third,zero);
        assert_eq!(neg_one_third + one_sixth,neg_one_sixth);
    }

    #[test]
    fn sub_isize_from_rational(){
        let two = Rational::new_rational(2,1);
        let three_halves = Rational::new_rational(3,2);
        let one = Rational::new_rational(1,1);
        let one_half = Rational::new_rational(1,2);
        let one_third = Rational::new_rational(1,3);
        let zero = Rational::new_rational_from_integers(0,1);
        let neg_one_half = Rational::new_rational_from_integers(-1,2);
        let neg_two_thirds = Rational::new_rational_from_integers(-2,3);
        let neg_one = Rational::new_rational_from_integers(-1,1);
        let neg_two = Rational::new_rational_from_integers(-2,1);
        assert_eq!(zero - (0 as isize),zero);
        assert_eq!(zero - (-1 as isize),one);
        assert_eq!(zero - (1 as isize),neg_one);
        assert_eq!(one - (0 as isize),one);
        assert_eq!(one - (-1 as isize),two);
        assert_eq!(one - (1 as isize),zero);
        assert_eq!(neg_one - (0 as isize),neg_one);
        assert_eq!(neg_one - (-1 as isize),zero);
        assert_eq!(neg_one - (1 as isize),neg_two);
        assert_eq!(neg_one_half - (-1 as isize),one_half);
        assert_eq!(neg_one_half - (-2 as isize),three_halves);
        assert_eq!(neg_one_half - (0 as isize),neg_one_half);
        assert_eq!(one_half - (0 as isize),one_half);
        assert_eq!(one_half - (-1 as isize),three_halves);
        assert_eq!(one_half - (1 as isize),neg_one_half);
        assert_eq!(three_halves - (2 as isize),neg_one_half);
        assert_eq!(one_third - (1 as isize),neg_two_thirds);
        assert_eq!(neg_two_thirds - (-1 as isize),one_third);
    }

    #[test]
    fn sub_usize_from_rational(){
        let one = Rational::new_rational(1,1);
        let one_half = Rational::new_rational(1,2);
        let one_third = Rational::new_rational(1,3);
        let zero = Rational::new_rational_from_integers(0,1);
        let neg_one_half = Rational::new_rational_from_integers(-1,2);
        let neg_two_thirds = Rational::new_rational_from_integers(-2,3);
        let neg_one = Rational::new_rational_from_integers(-1,1);
        let neg_two = Rational::new_rational_from_integers(-2,1);
        let neg_three_halves = Rational::new_rational_from_integers(-3,2);
        assert_eq!(zero - (0 as usize),zero);
        assert_eq!(zero - (1 as usize),neg_one);
        assert_eq!(one - (0 as usize),one);
        assert_eq!(one - (1 as usize),zero);
        assert_eq!(neg_one - (0 as usize),neg_one);
        assert_eq!(neg_one - (1 as usize),neg_two);
        assert_eq!(neg_one_half - (1 as usize),neg_three_halves);
        assert_eq!(neg_one_half - (0 as usize),neg_one_half);
        assert_eq!(one_half - (0 as usize),one_half);
        assert_eq!(one_half - (1 as usize),neg_one_half);
        assert_eq!(one_third - (1 as usize),neg_two_thirds);
    }

    #[test]
    fn mul_rational_to_rational(){
        let two = Rational::new_rational(2,1);
        let one = Rational::new_rational(1,1);
        let one_half = Rational::new_rational(1,2);
        let three_halves = Rational::new_rational(3,2);
        let one_third = Rational::new_rational(1,3);
        let two_thirds = Rational::new_rational(2,3);
        let one_fourth = Rational::new_rational(1,4);
        let neg_two = Rational::new_rational_from_integers(-2,1);
        let neg_one = Rational::new_rational_from_integers(-1,1);
        let neg_one_half = Rational::new_rational_from_integers(-1,2);
        let neg_one_fourth = Rational::new_rational_from_integers(-1,4);
        let one_sixth = Rational::new_rational_from_integers(1,6);
        let zero = Rational::new_rational_from_integers(0,1);
        assert_eq!(zero * zero , zero);
        assert_eq!(one * zero , zero);
        assert_eq!(zero * one , zero);
        assert_eq!(neg_one * zero , zero);
        assert_eq!(zero * neg_one , zero);
        assert_eq!(one * one , one);
        assert_eq!(one * neg_one , neg_one);
        assert_eq!(neg_one * one , neg_one);
        assert_eq!(neg_one * neg_one , one);
        assert_eq!(one_half * one_half , one_fourth);
        assert_eq!(neg_one_half * neg_one_half , one_fourth);
        assert_eq!(one_half * neg_one_half , neg_one_fourth);
        assert_eq!(neg_one_half * one_half , neg_one_fourth);
        assert_eq!(one_half * two , one);
        assert_eq!(two * one_half , one);
        assert_eq!(neg_one_half * two , neg_one);
        assert_eq!(two * neg_one_half , neg_one);
        assert_eq!(one_half * neg_two , neg_one);
        assert_eq!(neg_two * one_half , neg_one);
        assert_eq!(neg_one_half * neg_two , one);
        assert_eq!(neg_two * neg_one_half , one);
        assert_eq!(one_half * one_third , one_sixth);
        assert_eq!(three_halves * two_thirds , one);
        assert_eq!(two_thirds * three_halves , one);
    }

    #[test]
    fn mul_isize_to_rational(){
        let one = Rational::new_rational(1,1);
        let one_half = Rational::new_rational(1,2);
        let three_halves = Rational::new_rational(3,2);
        let one_third = Rational::new_rational(1,3);
        let two_thirds = Rational::new_rational(2,3);
        let neg_one = Rational::new_rational_from_integers(-1,1);
        let neg_one_half = Rational::new_rational_from_integers(-1,2);
        let zero = Rational::new_rational_from_integers(0,1);
        assert_eq!(zero * (0 as isize) , zero);
        assert_eq!(zero * (1 as isize) , zero);
        assert_eq!(zero * (-1 as isize) , zero);
        assert_eq!(one * (1 as isize) , one);
        assert_eq!(one * (0 as isize) , zero);
        assert_eq!(one * (-1 as isize) , neg_one);
        assert_eq!(neg_one * (1 as isize) , neg_one);
        assert_eq!(neg_one * (0 as isize) , zero);
        assert_eq!(neg_one * (-1 as isize) , one);
        assert_eq!(one_half * (1 as isize) , one_half);
        assert_eq!(one_half * (-1 as isize) , neg_one_half);
        assert_eq!(one_half * (2 as isize) , one);
        assert_eq!(one_half * (-2 as isize) , neg_one);
        assert_eq!(one_half * (3 as isize) , three_halves);
        assert_eq!(one_third * (2 as isize) , two_thirds);
        assert_eq!(one_third * (3 as isize) , one);
        assert_eq!(one_third * (-3 as isize) , neg_one);
    }

    #[test]
    fn mul_usize_to_rational(){
        let one = Rational::new_rational(1,1);
        let one_half = Rational::new_rational(1,2);
        let three_halves = Rational::new_rational(3,2);
        let one_third = Rational::new_rational(1,3);
        let two_thirds = Rational::new_rational(2,3);
        let neg_one = Rational::new_rational_from_integers(-1,1);
        let zero = Rational::new_rational_from_integers(0,1);
        assert_eq!(zero * (0 as isize) , zero);
        assert_eq!(zero * (1 as isize) , zero);
        assert_eq!(one * (1 as isize) , one);
        assert_eq!(one * (0 as isize) , zero);
        assert_eq!(neg_one * (1 as isize) , neg_one);
        assert_eq!(neg_one * (0 as isize) , zero);
        assert_eq!(one_half * (1 as isize) , one_half);
        assert_eq!(one_half * (2 as isize) , one);
        assert_eq!(one_half * (3 as isize) , three_halves);
        assert_eq!(one_third * (2 as isize) , two_thirds);
        assert_eq!(one_third * (3 as isize) , one);
    }

    #[test]
    fn div_rational_into_rational() {
        let zero = Rational::new_rational(0,1);
        let one = Rational::new_rational(1,1);
        let two = Rational::new_rational(2,1);
        let neg_one = Rational::new_signed_rational(1,1,true);
        let neg_two = Rational::new_signed_rational(2,1,true);
        let one_half = Rational::new_rational(1,2);
        let neg_one_half = Rational::new_signed_rational(1,2,true);
        let one_third = Rational::new_rational(1,3);
        let three_halves = Rational::new_rational(3,2);
        let two_thirds = Rational::new_rational(2,3);
        let one_fourth = Rational::new_rational(1,4);
        let four_ninths = Rational::new_rational(4,9);
        assert_eq!(zero/one,zero);
        assert_eq!(zero/neg_one,zero);
        assert_eq!(zero/two_thirds,zero);
        assert_eq!(one/one,one);
        assert_eq!(one/neg_one,neg_one);
        assert_eq!(neg_one/one,neg_one);
        assert_eq!(neg_one/neg_one,one);
        assert_eq!(one/two,one_half);
        assert_eq!(one/one_half,two);
        assert_eq!(one/neg_two,neg_one_half);
        assert_eq!(one/neg_one_half,neg_two);
        assert_eq!(two/one,two);
        assert_eq!(neg_two/one,neg_two);
        assert_eq!(three_halves/one,three_halves);
        assert_eq!(one_half/one_third,three_halves);
        assert_eq!(one_third/one_half,two_thirds);
        assert_eq!(one_half/one_fourth,two);
        assert_eq!(neg_one_half/one_fourth,neg_two);
        assert_eq!(two_thirds/four_ninths,three_halves);
    }

    #[test]
    fn operation_neg() {
        let one = Rational::new_rational(1,1);
        let neg_one = Rational::new_signed_rational(1,1,true);
        assert_eq!(-one,neg_one);
        assert_eq!(one,-neg_one);
        assert_eq!(-(-one),one);
        assert_eq!(neg_one,-(-neg_one));
    }
}