use std::ops::Add;
use std::ops::Mul;
use num_traits::pow;
use crate::curve_element::CurveElement;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: CurveElement,
    pub y: CurveElement,
    pub a: i32,
    pub b: i32
}

impl Point {

    // Setup new point
    pub fn new(x: CurveElement, y: CurveElement, a: i32, b: i32) -> Point {

        // x being None and y being None represents the point at infinity
        // Check for that here since the equation below won't make sense
        // with None values for both.
         
        if x.is_none() & y.is_none() {
            //This is the point at infinity
            return Point{x: x, y: y, a: a, b: b};
        }

        let nx = *(x.unwrap());
        let ny = *(y.unwrap());


        if pow(ny, 2) != pow(nx, 3) + a * nx + b {
            panic!("({}, {}) is not on the curve", nx, ny);
        }
        Point{x: x, y: y, a: a, b: b}
    }

    // View the point
    pub fn view(&self) -> String{
        format!("Point({},{})_{}_{}", self.x.to_string(), self.y.to_string(), self.a, self.b)
    }
}

// Implement == for 2 finite field elements
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {

        if (self.x == other.x) & (self.y == other.y) & (self.a == other.a) & (self.b == other.b) {
            true
        }else {
            false
        }
    }
}
impl Eq for Point {}

// Implement addition for 2 finite field elements
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if (self.a != other.a) | (self.b != other.b) {
            panic!("Points {:?}, {:?} are not on the same curve", self.view(), other.view())
        }

        // Case 0.0: self is the point at infinity, return other
        if self.x.is_none() {
            return other;
        }
        
        // Case 0.1: other is the point at infinity, return self
        if other.x.is_none() {
            return self;
        }

        // Case 1: self.x == other.x, self.y != other.y
        // Result is point at infinity
        if (self.x == other.x) & (self.y != other.y) {
            return Point{x: CurveElement::None, y: CurveElement::None, a: self.a, b: self.b};
        }

        // Case 2: self.x ≠ other.x
        // Formula (x3,y3)==(x1,y1)+(x2,y2)
        // s=(y2-y1)/(x2-x1)
        // x3=s**2-x1-x2
        // y3=s*(x1-x3)-y1

        if self.x != other.x {
            let sx = *(self.x.unwrap());
            let sy = *(self.y.unwrap());
            let ox = *(other.x.unwrap());
            let oy = *(other.y.unwrap());

            let s=(oy-sy)/(ox-sx);
            let x = pow(s, 2) - sx - ox;
            let y = s * (sx - x) - sy;
            return Point{x: CurveElement::int_to_curve(x), y: CurveElement::int_to_curve(y), a: self.a, b: self.b};
        }  
        
        // Case 4: if we are tangent to the vertical line,
        // we return the point at infinity
        // note instead of figuring out what 0 is for each type
        // we just use 0 * self.x

        if (self == other) & (*(self.y.unwrap()) == 0 * *(self.x.unwrap())) {
            return Point{x: CurveElement::None, y: CurveElement::None, a: self.a, b: self.b};
        } 
        
        // Case 3: self == other
        // Formula (x3,y3)=(x1,y1)+(x1,y1)
        // s=(3*x1**2+a)/(2*y1)
        // x3=s**2-2*x1
        // y3=s*(x1-x3)-y1
        if self == other {
            let sx = *(self.x.unwrap());
            let sy = *(self.y.unwrap());
            let s = (3 * pow(sx, 2) + self.a) / (2 * sy);
            let x = pow(s,2) - 2 * sx;
            let y = s * (sx - x) - sy;
            return Point{x: CurveElement::int_to_curve(x), y: CurveElement::int_to_curve(y), a: self.a, b: self.b};
        }
        panic!("Something wrong with the addition");
    }
}




//====================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    
    fn valid_point() {

        let i2c = CurveElement::int_to_curve;

        // On curve
        //let _x = Point::new(-2, 4, 5, 7);
        let _x = Point::new(i2c(3), i2c(-7), 5, 7); // should not raise an error
        let _x = Point::new(i2c(18), i2c(77), 5, 7); // should not raise an error

        // == implementation
        let p1 = Point::new(i2c(3), i2c(-7), 5, 7);
        let p2 = Point::new(i2c(3), i2c(-7), 5, 7);
        let p3 = Point::new(i2c(18), i2c(77), 5, 7);
        assert!(p1 == p2);
        assert!(p1 != p3);

        // Show point
        assert_eq!(p1.view(), String::from("Point(3,-7)_5_7"));

        // Test add 0
        let a = Point::new(CurveElement::None, CurveElement::None, 5, 7);
        let b = Point::new(i2c(2), i2c(5), 5, 7);
        let c = Point::new(i2c(2), i2c(-5), 5, 7);
        assert_eq!(a + b, b);
        assert_eq!(b + a, b);
        assert_eq!(b + c, a);

        // Test add 1
        let a = Point::new(i2c(3), i2c(7), 5, 7);
        let b = Point::new(i2c(-1), i2c(-1), 5, 7);
        assert_eq!(a + b, Point::new(i2c(2), i2c(-5), 5, 7));

        // Test add2
        let a = Point::new(i2c(-1), i2c(1), 5, 7);
        assert_eq!(a + a, Point::new(i2c(18), i2c(-77), 5, 7));

    }
}

