use rational;
use rational::Rational;

#[derive(Debug, Copy, Clone)]
pub struct RationalPoint2D {
    pub x: Rational,
    pub y: Rational,
}

#[derive(Debug, Copy, Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[derive(Debug, Copy, Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//The next three functions have the same setup and different only when the line has a non-zero non-ifinite slope.
//Possible refactoring???
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//Still needs to be (formally) tested.
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

//l_infinity_line
//Purpose:
//    Returns all integer points that are within .5 (using the L infinity metric) of the line given by the two 2D points.
//Pre-conditions:
//    None.
 pub fn l_infinity_line(point1: Point2D, point2: Point2D) -> Vec<Point2D>{
    let left_point;
    let right_point;
    let slope_sign;
    let mut squares: Vec<Point2D> = Vec::new();
    match point1.x.cmp(&point2.x){
        Ordering::Equal => {
            if let Ordering::Less = (point1.y).cmp(&point2.y) {
                for num in point1.y .. (point2.y + 1) {
                    squares.push(Point2D{ x: point1.x, y:num,});
                }
            } else {
                for num in point2.y .. (point1.y + 1) {
                    squares.push(Point2D{ x: point1.x, y:num,});
                }
            }
            return squares
        },
        Ordering::Less => {
            left_point = point1;
            right_point = point2;
        },
        Ordering::Greater => {
            left_point = point2;
            right_point = point1;
        },
    }
    match left_point.y.cmp(&right_point.y){
        Ordering::Equal => {
            for num in left_point.x .. (right_point.x + 1) {
                squares.push(Point2D{ x: num, y:left_point.y,});
            }
            return squares
        },
        Ordering::Less => {
            slope_sign = 1;
        },
        Ordering::Greater => {
            slope_sign = -1;
        },
    }
    let slope = Rational::new_rational_from_integers((right_point.y-left_point.y).abs(),right_point.x-left_point.x);
    let mut current_height = 0;
    let mut kitty_coner: bool;
    for num in 0 .. right_point.x-left_point.x {
        let mut line_intersection_between_integers = slope*Rational::new_rational_from_integers(num*2+1,2);
        if line_intersection_between_integers.denominator() == 2 {
            kitty_coner = true;
        }else{
            kitty_coner = false;
        }
        let last_y_square_hit = line_intersection_between_integers.rational_ceil(2).floor();
        for y_cords in current_height .. (last_y_square_hit + 1) {
            squares.push(Point2D{ x: num + left_point.x, y:slope_sign*y_cords + left_point.y,});
        }
        current_height = last_y_square_hit;
        if kitty_coner {
            current_height += 1;
        }
    }
    for y_cords in current_height .. (right_point.y-left_point.y).abs()+1 {
        squares.push(Point2D{ x: right_point.x, y:slope_sign*y_cords + left_point.y,});
    }
    squares
}

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//Still needs to be tested.
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

//l_infinity_line_rational
//Purpose:
//    Returns all integer points that are within the given rational radius(using the L infinity metric) of the line given by the two 2D points.
//Pre-conditions:
//    The rational radius is expected to be between 0 and .5 inclusive.
//    If the radius is less than 0, it is treated as 0.
//    If the radius is greater than .5, it it treated as .5.
pub fn l_infinity_line_rational(point1: Point2D, point2: Point2D, radius: Rational) -> Vec<Point2D>{
    let left_point;
    let right_point;
    let slope_sign;
    let mut squares: Vec<Point2D> = Vec::new();

    if radius <= Rational::new_rational_from_integer(0) {
        return integer_points_on_line(point1, point2);
    } else if radius >= Rational::new_rational_from_integer(1) {
        return l_infinity_line(point1, point2);
    }


    match point1.x.cmp(&point2.x){
        Ordering::Equal => {
            if let Ordering::Less = (point1.y).cmp(&point2.y) {
                for num in point1.y .. (point2.y + 1) {
                    squares.push(Point2D{ x: point1.x, y:num,});
                }
            } else {
                for num in point2.y .. (point1.y + 1) {
                    squares.push(Point2D{ x: point1.x, y:num,});
                }
            }
            return squares
        },
        Ordering::Less => {
            left_point = point1;
            right_point = point2;
        },
        Ordering::Greater => {
            left_point = point2;
            right_point = point1;
        },
    }
    match left_point.y.cmp(&right_point.y){
        Ordering::Equal => {
            for num in left_point.x .. (right_point.x + 1) {
                squares.push(Point2D{ x: num, y:left_point.y,});
            }
            return squares
        },
        Ordering::Less => {
            slope_sign = 1;
        },
        Ordering::Greater => {
            slope_sign = -1;
        },
    }
    let slope = Rational::new_rational_from_integers((right_point.y-left_point.y).abs(),right_point.x-left_point.x);
    for y_cords in 0 .. (slope*radius + radius).ceil() {
        squares.push(Point2D{ x: left_point.x, y:slope_sign*y_cords + left_point.y,});
    }
    for num in 1 .. right_point.x-left_point.x {
        for y_cords in ((slope*(-radius + num)).floor() + 1) .. (slope*(radius + num)).ceil() { //order of operations for rationals and usizes requires that the rational go first.
            squares.push(Point2D{ x: num + left_point.x, y:slope_sign*y_cords + left_point.y,});
        }
    }
    for y_cords in ((slope*(-radius + right_point.x - left_point.x)).floor() + 1) .. (right_point.y-left_point.y).abs()+1 {
        squares.push(Point2D{ x: right_point.x, y:slope_sign*y_cords + left_point.y,});
    }
    squares

}

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//Still needs to be (formally) tested.
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

//integer_points_on_line
//Purpose:
//    Returns all integer points that are  on the line given by the two 2D points.
//Pre-conditions:
//    None.
pub fn integer_points_on_line(point1: Point2D, point2: Point2D) -> Vec<Point2D>{
    let left_point;
    let right_point;
    let mut squares: Vec<Point2D> = Vec::new();
    match point1.x.cmp(&point2.x){
        Ordering::Equal => {
            if let Ordering::Less = (point1.y).cmp(&point2.y) {
                for num in point1.y .. (point2.y + 1) {
                    squares.push(Point2D{ x: point1.x, y:num,});
                }
            } else {
                for num in point2.y .. (point1.y + 1) {
                    squares.push(Point2D{ x: point1.x, y:num,});
                }
            }
            return squares
        },
        Ordering::Less => {
            left_point = point1;
            right_point = point2;
        },
        Ordering::Greater => {
            left_point = point2;
            right_point = point1;
        },
    }
    if let Ordering::Equal = (left_point.y).cmp(&right_point.y) {
        for num in left_point.x .. (right_point.x + 1) {
            squares.push(Point2D{ x: num, y:left_point.y,});
        }
        return squares
    }
    let rise = right_point.y-left_point.y;
    let run:usize = (right_point.x-left_point.x) as usize;
    let common_factor = rational::gcd(run,rise.abs() as usize); //rise might be negaitive.
    let reduced_run = run/common_factor;
    let reduced_rise = rise/(common_factor as isize);
    for num in 0 .. (run/reduced_run + 1) {
        squares.push(Point2D{ x: left_point.x + ((num*reduced_run) as isize), y:left_point.y + reduced_rise*(num as isize),});
    }
    squares
}

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//Still needs to be tested.
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

//l_infinity_line_3D
//Purpose:
//    Returns all integer points that are within .5 (using the L infinity metric) of the line given by the two 2D points.
//Pre-conditions:
//    None.
pub fn l_infinity_line_3D(point1: Point3D, point2: Point3D) -> Vec<Point3D>{
    let left_point;
    let right_point;
    let slope_sign;
    let mut cubes: Vec<Point3D> = Vec::new();
    let projected_cubes = l_infinity_line(Point2D{ x: point1.x, y: point1.y,},Point2D{ x: point2.x, y: point2.y,});
    if (projected_cubes[0].x == point1.x) & (projected_cubes[0].y == point1.y) {
        left_point = point1;
        right_point = point2;
    } else{
        left_point = point2;
        right_point = point1;
    }
    match left_point.z.cmp(&right_point.z){
        Ordering::Equal => {
            for projected_cube in projected_cubes {
                cubes.push(Point3D{ x: projected_cube.x, y: projected_cube.y, z: left_point.z});
            }
            return cubes;
        },
        Ordering::Less => {
            slope_sign = 1;
        },
        Ordering::Greater => {
            slope_sign = -1;
        },
    }

    let slope = Rational::new_rational_from_integers((right_point.y-left_point.y).abs(),right_point.x-left_point.x);
    let mut current_height = 0;
    let mut kitty_coner: bool;
    for num in 0 .. right_point.x-left_point.x {
        let mut line_intersection_between_integers = slope*Rational::new_rational_from_integers(num*2+1,2);
        if line_intersection_between_integers.denominator() == 2 {
            kitty_coner = true;
        }else{
            kitty_coner = false;
        }
        let last_y_square_hit = line_intersection_between_integers.rational_ceil(2).floor();
        for y_cords in current_height .. (last_y_square_hit + 1) {
            cubes.push(Point3D{ x: num + left_point.x, y:slope_sign*y_cords + left_point.y, z:0,});
        }
        current_height = last_y_square_hit;
        if kitty_coner {
            current_height += 1;
        }
    }
    for y_cords in current_height .. (right_point.y-left_point.y).abs()+1 {
        cubes.push(Point3D{ x: right_point.x, y:slope_sign*y_cords + left_point.y, z:0,});
    }
    cubes
}

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//Still needs to be implemented.
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

//integer_points_on_line_3D
//Purpose:
//    Returns all integer points that are  on the line given by the two 3D points.
//Pre-conditions:
//    None.
pub fn integer_points_on_line_3D(point1: Point3D, point2: Point3D) -> Vec<Point3D>{
    let left_point;
    let right_point;
    let mut cubes: Vec<Point3D> = Vec::new();
    if point1.x == point2.x {
        let projected_cubes = integer_points_on_line(Point2D{x: point1.y, y: point1.z}, Point2D{x: point2.y, y: point2.z});
        for square in projected_cubes {
            cubes.push(Point3D{ x:point1.x, y: square.x, z: square.y})
        }
        return cubes;
    }
    if point1.y == point2.y {
        let projected_cubes = integer_points_on_line(Point2D{x: point1.x, y: point1.z}, Point2D{x: point2.x, y: point2.z});
        for square in projected_cubes {
            cubes.push(Point3D{ x:square.x, y: point1.y, z: square.y})
        }
        return cubes;
    }
    if point1.z == point2.z {
        let projected_cubes = integer_points_on_line(Point2D{x: point1.x, y: point1.y}, Point2D{x: point2.x, y: point2.y});
        for square in projected_cubes {
            cubes.push(Point3D{ x:square.x, y: square.y, z: point1.z})
        }
        return cubes;
    }
    //At this point we know that the parameterized line has no zero slopes with respect to time, which means we don't have to worry about divison by zero.
    match point1.x.cmp(&point2.x){
        Ordering::Less => {
            left_point = point1;
            right_point = point2;
        },
        _ => { //Since they can't be equal, if we are here than the returned object is Ordering::Greater
            left_point = point2;
            right_point = point1;
        },
    }
    let dydx = Rational::new_rational_from_integers(right_point.y-left_point.y,right_point.x-left_point.x);
    let dzdx = Rational::new_rational_from_integers(right_point.z-left_point.z,right_point.x-left_point.x);
    let lcm = ((dydx.denominator()/rational::gcd(dydx.denominator(),dzdx.denominator()))*dzdx.denominator()) as isize;
    for num in 0 .. (right_point.x-left_point.x)/lcm + 1 {
        cubes.push(Point3D{ x: num*lcm + left_point.x, y: (dydx*num*lcm).floor() as isize + left_point.y, z: (dzdx*num*lcm).floor() as isize + left_point.z})
    }
    cubes
}

//integer_points_inside_l_2_circle_slow
//Purpose:
//    Returns all integer points that are within the given radius (using the L 2 metric) from the center.
//Pre-conditions:
//    None
//Notes:
//    This does it the slow way and check every integer tuple would could be in range
//    Meant to validate integer_points_inside_l_2_circle
pub fn integer_points_inside_l_2_circle_slow(center: Point2D, radius: isize) -> Vec<Point2D>{
    let mut squares: Vec<Point2D> = Vec::new();
    for x_cords in -radius .. radius +1 {
        for y_cords in -radius .. radius +1 {
            if ((x_cords*x_cords) + (y_cords*y_cords)) <= radius*radius{
                squares.push(Point2D{ x: x_cords + center.x, y: y_cords + center.y,});
            }
        }
    }
    squares
}

//integer_points_inside_l_2_circle
//Purpose:
//    Returns all integer points that are within the given radius (using the L 2 metric) from the center.
//Pre-conditions:
//    None
//Notes:
//    This is the standard Euclidean metric, so squart root is needed.
pub fn integer_points_inside_l_2_circle(center: Point2D, radius: isize) -> Vec<Point2D>{
    let mut squares: Vec<Point2D> = Vec::new();
    let farthest_digonal_integer = ((radius as f32)*(radius as f32)/2.0).sqrt() as isize;
    for x_cords in -farthest_digonal_integer .. farthest_digonal_integer +1 {
        for y_cords in -farthest_digonal_integer .. farthest_digonal_integer +1 {
            squares.push(Point2D{ x: x_cords + center.x, y: y_cords + center.y,});
        }
    }
    for num in farthest_digonal_integer + 1 .. radius + 1 {
        squares.push(Point2D{ x: num + center.x, y: center.y,});
        squares.push(Point2D{ x: -num + center.x, y: center.y,});
        squares.push(Point2D{ x: center.x, y: num + center.y,});
        squares.push(Point2D{ x: center.x, y: -num + center.y,});
    }
    for x_cord in farthest_digonal_integer + 1 .. radius {
        for y_cord in 1 .. (((radius*radius - x_cord*x_cord) as f32).sqrt() as isize) + 1 {
            squares.push(Point2D{ x: x_cord + center.x, y: y_cord + center.y,});
            squares.push(Point2D{ x: x_cord + center.x, y: -y_cord + center.y,});
            squares.push(Point2D{ x: -x_cord + center.x, y: y_cord + center.y,});
            squares.push(Point2D{ x: -x_cord + center.x, y: -y_cord + center.y,});
            squares.push(Point2D{ x: y_cord + center.x, y: x_cord + center.y,});
            squares.push(Point2D{ x: -y_cord + center.x, y: x_cord + center.y,});
            squares.push(Point2D{ x: y_cord + center.x, y: -x_cord + center.y,});
            squares.push(Point2D{ x: -y_cord + center.x, y: -x_cord + center.y,});
        }
    }
    squares
}

//integer_points_inside_l_1_circle
//Purpose:
//    Returns all integer points that are within the given radius (using the L 1 metric) from the center.
//Pre-conditions:
//    None
//Notes:
//    Since this is the L 1 metric only loops are needed. This is also know as the taxi-cab metric.
pub fn integer_points_inside_l_1_circle(center: Point2D, radius: isize) -> Vec<Point2D>{
    let mut squares: Vec<Point2D> = Vec::new();
    for x_cords in -radius .. radius +1 {
        for y_cords in (-radius + x_cords.abs()) .. radius - x_cords.abs() + 1 {
            squares.push(Point2D{ x: x_cords + center.x, y: y_cords + center.y,});
        }
    }
    squares
}

//integer_points_inside_l_infinity_circle
//Purpose:
//    Returns all integer points that are within the given radius (using the L infinity metric) from the center.
//Pre-conditions:
//    None
//Notes:
//    Since this is the L infinity metric only loops are needed.
pub fn integer_points_inside_l_infinity_circle(center: Point2D, radius: isize) -> Vec<Point2D>{
    let mut squares: Vec<Point2D> = Vec::new();
    for x_cords in -radius .. radius +1 {
        for y_cords in -radius .. radius +1 {
            squares.push(Point2D{ x: x_cords + center.x, y: y_cords + center.y,});
        }
    }
    squares
}

//integer_points_inside_l_2_sphere_slow
//Purpose:
//    Returns all integer points that are within the given radius (using the L 2 metric) from the center.
//Pre-conditions:
//    None
//Notes:
//    This does it the slow way and check every integer triple would could be in range and uses the square root
//    Meant to validate integer_points_inside_l_2_sphere
pub fn integer_points_inside_l_2_sphere_slow(center: Point3D, radius: isize) -> Vec<Point3D>{
    let mut cubes: Vec<Point3D> = Vec::new();
    for x_cords in -radius .. radius +1 {
        for y_cords in -radius .. radius +1 {
            for z_cords in -radius .. radius +1 {
                if ((x_cords*x_cords) + (y_cords*y_cords) + (z_cords*z_cords)) <= radius*radius{
                    cubes.push(Point3D{ x: x_cords + center.x, y: y_cords + center.y, z: z_cords + center.z,});
                }
            }
        }
    }
    cubes
}

//integer_points_inside_l_2_sphere
//Purpose:
//    Returns all integer points that are within the given radius (using the L 2 metric) from the center.
//Pre-conditions:
//    None
//Notes:
//    This is the standard Euclidean metric, so squart root is needed.
pub fn integer_points_inside_l_2_sphere(center: Point3D, radius: isize) -> Vec<Point3D>{
    let mut cubes: Vec<Point3D> = Vec::new();
    let farthest_digonal_integer = ((radius as f32)*(radius as f32)/3.0).sqrt() as isize;
    for x_cords in -farthest_digonal_integer .. farthest_digonal_integer +1 {
        for y_cords in -farthest_digonal_integer .. farthest_digonal_integer +1 {
            for z_cords in -farthest_digonal_integer .. farthest_digonal_integer +1 {
                cubes.push(Point3D{ x: x_cords + center.x, y: y_cords + center.y, z: z_cords + center.z,});
            }
        }
    }
    for num in farthest_digonal_integer + 1 .. radius {
        let squares = integer_points_inside_l_2_circle(Point2D{ x: 0, y: 0,}, ((radius*radius - num*num) as f32).sqrt() as isize);
        for square in squares {
            cubes.push(Point3D{ x: center.x + num, y: square.x + center.y, z: square.y + center.z,});
            cubes.push(Point3D{ x: center.x - num, y: square.x + center.y, z: square.y + center.z,});
            cubes.push(Point3D{ x: square.x + center.x, y: center.y + num, z: square.y + center.z,});
            cubes.push(Point3D{ x: square.x + center.x, y: center.y - num, z: square.y + center.z,});
            cubes.push(Point3D{ x: square.x + center.x, y: square.y + center.y, z: center.z + num,});
            cubes.push(Point3D{ x: square.x + center.x, y: square.y + center.y, z: center.z - num,});
        }
    }
    if radius > 0 {
        cubes.push(Point3D{ x: center.x + radius, y: center.y, z: center.z,});
        cubes.push(Point3D{ x: center.x - radius, y: center.y, z: center.z,});
        cubes.push(Point3D{ x: center.x, y: center.y + radius, z: center.z,});
        cubes.push(Point3D{ x: center.x, y: center.y - radius, z: center.z,});
        cubes.push(Point3D{ x: center.x, y: center.y, z: center.z + radius,});
        cubes.push(Point3D{ x: center.x, y: center.y, z: center.z - radius,});
    }
    cubes
}

//integer_points_inside_l_1_sphere
//Purpose:
//    Returns all integer points that are within the given radius (using the L 1 metric) from the center.
//Pre-conditions:
//    None
//Notes:
//    Since this is the L 1 metric only loops are needed. This is also know as the taxi-cab metric.
pub fn integer_points_inside_l_1_sphere(center: Point3D, radius: isize) -> Vec<Point3D>{
    let mut cubes: Vec<Point3D> = Vec::new();
    for x_cords in -radius .. radius +1 {
        for y_cords in (-radius + x_cords.abs()) .. radius - x_cords.abs() + 1 {
            for z_cords in (-radius + x_cords.abs() + y_cords.abs()) .. radius - x_cords.abs() - y_cords.abs() + 1 {
                cubes.push(Point3D{ x: x_cords + center.x, y: y_cords + center.y, z: z_cords + center.z,});
            }
        }
    }
    cubes
}

//integer_points_inside_l_infinity_sphere
//Purpose:
//    Returns all integer points that are within the given radius (using the L infinity metric) from the center.
//Pre-conditions:
//    None
//Notes:
//    Since this is the L infinity metric only loops are needed.
pub fn integer_points_inside_l_infinity_sphere(center: Point3D, radius: isize) -> Vec<Point3D>{
    let mut cubes: Vec<Point3D> = Vec::new();
    for x_cords in -radius .. radius + 1 {
        for y_cords in -radius .. radius + 1 {
            for z_cords in -radius .. radius + 1 {
                cubes.push(Point3D{ x: x_cords + center.x, y: y_cords + center.y, z: z_cords + center.z,});
            }
        }
    }
    cubes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Points_in_sphere(){
        assert_eq!(integer_points_inside_l_2_sphere_slow(Point3D{ x: 0, y: 0, z: 0,},0).sort(),integer_points_inside_l_2_sphere(Point3D{ x: 0, y: 0, z: 0,},0).sort());
        assert_eq!(integer_points_inside_l_2_sphere_slow(Point3D{ x: 0, y: 0, z: 0,},1).sort(),integer_points_inside_l_2_sphere(Point3D{ x: 0, y: 0, z: 0,},1).sort());
        assert_eq!(integer_points_inside_l_2_sphere_slow(Point3D{ x: 0, y: 0, z: 0,},10).sort(),integer_points_inside_l_2_sphere(Point3D{ x: 0, y: 0, z: 0,},10).sort());
        assert_eq!(integer_points_inside_l_2_sphere_slow(Point3D{ x: 0, y: 0, z: 0,},100).sort(),integer_points_inside_l_2_sphere(Point3D{ x: 0, y: 0, z: 0,},100).sort());
    }

    #[test]
    fn Points_in_circle(){
        assert_eq!(integer_points_inside_l_2_circle_slow(Point2D{ x: 0, y: 0,},0).sort(),integer_points_inside_l_2_circle(Point2D{ x: 0, y: 0,},0).sort());
        assert_eq!(integer_points_inside_l_2_circle_slow(Point2D{ x: 0, y: 0,},1).sort(),integer_points_inside_l_2_circle(Point2D{ x: 0, y: 0,},1).sort());
        assert_eq!(integer_points_inside_l_2_circle_slow(Point2D{ x: 0, y: 0,},10).sort(),integer_points_inside_l_2_circle(Point2D{ x: 0, y: 0,},10).sort());
        assert_eq!(integer_points_inside_l_2_circle_slow(Point2D{ x: 0, y: 0,},100).sort(),integer_points_inside_l_2_circle(Point2D{ x: 0, y: 0,},100).sort());
    }
}