//Author: Everett Sullivan.
//Date created: March 14th 2019
//Purpose: Contains a rational struct for doing computations on rationals
//TO DO: creating testing function for integer_cells_on_line_segment_3d
//       create corrasponding functions for a hex grid.

use std::cmp;
use std::cmp::Ordering;

mod rational;
use rational::Rational;

#[derive(Debug, Copy, Clone)]
struct RationalPoint2D {
    x: Rational,
    y: Rational,
}

#[derive(Debug, Copy, Clone,PartialEq,Eq,PartialOrd,Ord)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Copy, Clone,PartialEq,Eq,PartialOrd,Ord)]
struct Point2D {
    x: isize,
    y: isize,
}

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//The next three functions have the same setup and different only when the line has a non-zero non-ifinite slope.
//Possible refactoring???
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

//integer_cells_on_line_segment_2d
//Purpose:
//    Returns all integer cells (x,y) from a grid that lie on the line segment given by the two 2D points.
//    Which is to say it returns all integer points (x,y) that are within .5 (using the L infinity metric) of the line segment given by the two 2D points.
//Pre-conditions:
//    None.
 fn integer_cells_on_line_segment_2d(point1: Point2D, point2: Point2D) -> Vec<Point2D>{
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
        let line_intersection_between_integers = slope*Rational::new_rational_from_integers(num*2+1,2);
        if line_intersection_between_integers.denominator() == 2 {
            kitty_coner = true;
        }else{
            kitty_coner = false;
        }
        let last_y_square_hit = line_intersection_between_integers.rational_ceil(2).floor();
        for y_cord in current_height .. (last_y_square_hit + 1) {
            squares.push(Point2D{ x: num + left_point.x, y:slope_sign*y_cord + left_point.y,});
        }
        current_height = last_y_square_hit;
        if kitty_coner {
            current_height += 1;
        }
    }
    for y_cord in current_height .. (right_point.y-left_point.y).abs()+1 {
        squares.push(Point2D{ x: right_point.x, y:slope_sign*y_cord + left_point.y,});
    }
    squares
}

//integer_cells_on_line_segment_2d_rational_scale
//Purpose:
//    Returns all integer cells (x,y) from a grid that lie on the line segment given by the two 2D points.
//    All cells are scaled by the given radius.
//    Which is to say it returns all integer points (x,y) that are within the given rational radius(using the L infinity metric)
//    of the line segment given by the two 2D points.
//Pre-conditions:
//    None
//Notes:
//    The rational radius is expected to be between 0 and .5 inclusive.
//    If the radius is less than 0, it is treated as 0.
//    If the radius is greater than .5, it it treated as .5.
fn integer_cells_on_line_segment_2d_rational_scale(point1: Point2D, point2: Point2D, radius: Rational) -> Vec<Point2D>{
    let left_point;
    let right_point;
    let slope_sign;
    let mut squares: Vec<Point2D> = Vec::new();

    if radius <= Rational::new_rational_from_integer(0) {
        return integer_points_on_line_segment_2d(point1, point2);
    } else if radius >= Rational::new_rational(1,2) {
        return integer_cells_on_line_segment_2d(point1, point2);
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
    for y_cord in 0 .. (slope*radius + radius).ceil() {
        squares.push(Point2D{ x: left_point.x, y:slope_sign*y_cord + left_point.y,});
    }
    for num in 1 .. right_point.x-left_point.x {
        for y_cord in ((slope*(-radius + num) - radius).floor() + 1) .. (slope*(radius + num) + radius).ceil() { //order of operations for rationals and usizes requires that the rational go first.
            squares.push(Point2D{ x: num + left_point.x, y:slope_sign*y_cord + left_point.y,});
        }
    }
    for y_cord in ((slope*((-radius) + right_point.x - left_point.x) - radius).floor() + 1) .. (right_point.y-left_point.y).abs()+1 {
        squares.push(Point2D{ x: right_point.x, y:slope_sign*y_cord + left_point.y,});
    }
    squares

}

//integer_points_on_line_segment_2d
//Purpose:
//    Returns all integer points (x,y) that are  on the line segment given by the two 2D points.
//Pre-conditions:
//    None.
fn integer_points_on_line_segment_2d(point1: Point2D, point2: Point2D) -> Vec<Point2D>{
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


//l_infinity_line_rational_endpoints
//Purpose:
//    Returns all integer points that are within .5 (using the L infinity metric) of the line given by the two 2D points.
//Pre-conditions:
//    None.
 fn l_infinity_line_rational_endpoints(point1: RationalPoint2D, point2: RationalPoint2D) -> Vec<Point2D>{
    let mut left_point;
    let mut right_point;
    let left_x;
    let right_x;
    let lower_y;
    let upper_y;
    let y_flip: bool;
    let base_height;
    let mut squares: Vec<Point2D> = Vec::new();
    match point1.x.cmp(&point2.x){
        Ordering::Equal => {
            if point1.x.denominator() == 2 { // if we are between squares, nothing is hit.
                return squares
            } else if let Ordering::Less = (point1.y).cmp(&point2.y) {
                lower_y = point1.y.rational_floor(2).ceil();
                upper_y = point2.y.rational_ceil(2).floor();
            } else {
                lower_y = point2.y.rational_floor(2).ceil();
                upper_y = point1.y.rational_ceil(2).floor();
            }
            for num in lower_y .. upper_y+1 { // in the case where lower_y and upper_y are equal and of the from a/2, nothing will be added, as intended.
                let closest_integer_x = point1.x.rational_floor(2).ceil();
                squares.push(Point2D{ x: closest_integer_x, y:num,});
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
    left_x = left_point.x.rational_floor(2).ceil();
    right_x = right_point.x.rational_ceil(2).floor();
    match left_point.y.cmp(&right_point.y){
        Ordering::Equal => {
            if point1.y.denominator() == 2 { // if we are between squares, nothing is hit.
                return squares
            } else {
                for num in left_x .. (right_x + 1) {
                    squares.push(Point2D{ x: num, y:point1.y.rational_ceil(2).floor(),});
                }
            }
            return squares
        },
        Ordering::Less => {
            y_flip = false;
        },
        Ordering::Greater => {
            y_flip = true;
            left_point.y = -left_point.y;
            right_point.y = -right_point.y;
        },
    }
    lower_y = left_point.y.rational_floor(2).ceil();
    upper_y = right_point.y.rational_ceil(2).floor();
    let slope = (right_point.y-left_point.y)/(right_point.x-left_point.x);
    let mut current_height;
    let final_height;
    base_height = lower_y;
    final_height = upper_y;
    current_height = base_height;
    let mut kitty_coner: bool;
    for num in 0 .. (right_x-left_x) {
        let line_intersection_between_integers = slope*Rational::new_rational_from_integers(num*2+1,2) + base_height;
        if line_intersection_between_integers.denominator() == 2 {
            kitty_coner = true;
        }else{
            kitty_coner = false;
        }
        let last_y_square_hit = line_intersection_between_integers.rational_ceil(2).floor();
        for y_cords in current_height .. (last_y_square_hit + 1) {
            squares.push(Point2D{ x: num + left_x, y: y_cords,});
        }
        current_height = last_y_square_hit;
        if kitty_coner {
            current_height += 1;
        }
    }
    for y_cords in current_height .. (final_height).abs()+1 {
        squares.push(Point2D{ x: right_x, y: y_cords,});
    }
    if y_flip {
        let mut squares_2: Vec<Point2D> = Vec::new();
        for square in squares{
            squares_2.push(Point2D{ x: square.x, y: -square.y,});
        }
        squares = squares_2;
    }
    squares
}

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//Still needs to be tested.
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

//integer_cells_on_line_segment_3d
//Purpose:
//    Returns all integer cells (x,y,z) from a grid that lie on the line segment given by the two 3D points.
//    Which is to say it returns all integer points (x,y,z) that are within .5 (using the L infinity metric) of the line segment given by the two 3D points.
//Pre-conditions:
//    None.
fn integer_cells_on_line_segment_3d(point1: Point3D, point2: Point3D) -> Vec<Point3D>{
    let mut left_point;
    let mut right_point;
    let z_flip: bool;
    let mut cubes: Vec<Point3D> = Vec::new();
    let mut squares: Vec<Point2D>;
    if point1.x < point2.x {
        left_point = point1;
        right_point = point2;
    } else{
        left_point = point2;
        right_point = point1;
    }
    match left_point.z.cmp(&right_point.z){
        Ordering::Equal => {
            squares = integer_cells_on_line_segment_2d(Point2D{ x: left_point.x, y: left_point.y,},Point2D{ x: right_point.x, y: right_point.y,});
            for square in squares{
                cubes.push(Point3D{ x: square.x, y: square.y, z: left_point.z});
            }
            return cubes
        },
        Ordering::Less => {
            z_flip = false;
        },
        Ordering::Greater => {
            z_flip = true;
            left_point.z = -left_point.z;
            right_point.z = -right_point.z;
        },
    }

    let dxdz = Rational::new_rational_from_integers(right_point.x - left_point.x,right_point.z - left_point.z);
    let dydz = Rational::new_rational_from_integers(right_point.y - left_point.y,right_point.z - left_point.z);
    let mut current_rational_point = RationalPoint2D{x: Rational::new_rational_from_integer(left_point.x), y: Rational::new_rational_from_integer(left_point.y),};
    let mut next_rational_point;
    for num in 0 .. (right_point.z - left_point.z) {
        let x_hit = (Rational::new_rational_from_integers(num*2+1,2) * dxdz) + left_point.x;
        let y_hit = (Rational::new_rational_from_integers(num*2+1,2) * dydz) + left_point.y;
        next_rational_point = RationalPoint2D{x: x_hit, y: y_hit,};
        squares = l_infinity_line_rational_endpoints(current_rational_point,next_rational_point);
        for square in squares {
            cubes.push(Point3D{ x: square.x, y: square.y, z: left_point.z + num});
        }
        current_rational_point = next_rational_point;
    }
    next_rational_point = RationalPoint2D{x: Rational::new_rational_from_integer(right_point.x), y: Rational::new_rational_from_integer(right_point.y),};
    let squares = l_infinity_line_rational_endpoints(current_rational_point,next_rational_point);
    for square in squares {
        cubes.push(Point3D{ x: square.x, y: square.y, z: right_point.z});
    }
    if z_flip {
        let mut flipped_cubes: Vec<Point3D> = Vec::new();
        for cube in cubes {
            flipped_cubes.push(Point3D{x: cube.x, y: cube.y, z: -cube.z,});
        }
        cubes = flipped_cubes;
    }
    cubes
}

//integer_points_on_line_segment_3d
//Purpose:
//    Returns all integer points (x,y,z) that are  on the line segment given by the two 3D points.
//Pre-conditions:
//    None.
fn integer_points_on_line_segment_3d(point1: Point3D, point2: Point3D) -> Vec<Point3D>{
    let left_point;
    let right_point;
    let mut cubes: Vec<Point3D> = Vec::new();
    if point1.x == point2.x {
        let projected_cubes = integer_points_on_line_segment_2d(Point2D{x: point1.y, y: point1.z}, Point2D{x: point2.y, y: point2.z});
        for square in projected_cubes {
            cubes.push(Point3D{ x:point1.x, y: square.x, z: square.y})
        }
        return cubes;
    }
    if point1.y == point2.y {
        let projected_cubes = integer_points_on_line_segment_2d(Point2D{x: point1.x, y: point1.z}, Point2D{x: point2.x, y: point2.z});
        for square in projected_cubes {
            cubes.push(Point3D{ x:square.x, y: point1.y, z: square.y})
        }
        return cubes;
    }
    if point1.z == point2.z {
        let projected_cubes = integer_points_on_line_segment_2d(Point2D{x: point1.x, y: point1.y}, Point2D{x: point2.x, y: point2.y});
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

//integer_points_inside_circle_slow
//Purpose:
//    Returns all integer points (x,y) that are within the given radius from the center.
//Pre-conditions:
//    radius is non-negaitive.
//Notes:
//    This does it the slow way and check every integer tuple would could be in range
//    Meant to validate integer_points_inside_circle
fn integer_points_inside_circle_slow(center: Point2D, radius: isize) -> Vec<Point2D>{
    let mut squares: Vec<Point2D> = Vec::new();
    for x_cord in -radius .. radius +1 {
        for y_cord in -radius .. radius +1 {
            if ((x_cord*x_cord) + (y_cord*y_cord)) <= radius*radius{
                squares.push(Point2D{ x: x_cord + center.x, y: y_cord + center.y,});
            }
        }
    }
    squares
}

//integer_points_inside_circle
//Purpose:
//    Returns all integer points (x,y) that are within the given radius from the center.
//Pre-conditions:
//    radius is non-negaitive.
//Notes:
//    This is the standard Euclidean metric, so squart root is needed.
fn integer_points_inside_circle(center: Point2D, radius: isize) -> Vec<Point2D>{
    let mut squares: Vec<Point2D> = Vec::new();
    let farthest_digonal_integer = ((radius as f32)*(radius as f32)/2.0).sqrt() as isize;
    for x_cord in -farthest_digonal_integer .. farthest_digonal_integer +1 {
        for y_cord in -farthest_digonal_integer .. farthest_digonal_integer +1 {
            squares.push(Point2D{ x: x_cord + center.x, y: y_cord + center.y,});
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

//integer_points_inside_dimond_2d
//Purpose:
//    Returns all integers points (x,y) such that |x -center.x| + |y - center.y| <= radius.
//    Equivalent to returing all integer points (x,y) that are within the given radius (using the L 1 metric) from the center.
//Pre-conditions:
//    radius is non-negative.
//Notes:
//    Since this is the L 1 metric only loops are needed. This is also know as the taxi-cab metric.
fn integer_points_inside_dimond_2d(center: Point2D, radius: isize) -> Vec<Point2D>{
    let mut squares: Vec<Point2D> = Vec::new();
    for x_cord in -radius .. radius +1 {
        for y_cord in (-radius + x_cord.abs()) .. radius - x_cord.abs() + 1 {
            squares.push(Point2D{ x: x_cord + center.x, y: y_cord + center.y,});
        }
    }
    squares
}

//integer_points_inside_square
//Purpose:
//    Returns all integers points (x,y) such that max(|x -center.x|,|y - center.y|) <= radius.
//    Equivalent to returing all integer points that are within the given radius (using the L infinity metric) from the center.
//Pre-conditions:
//    radius is non-negative.
//Notes:
//    Since this is the L infinity metric only loops are needed.
fn integer_points_inside_square(center: Point2D, radius: isize) -> Vec<Point2D>{
    let mut squares: Vec<Point2D> = Vec::new();
    for x_cord in -radius .. radius + 1 {
        for y_cord in -radius .. radius + 1 {
            squares.push(Point2D{ x: x_cord + center.x, y: y_cord + center.y,});
        }
    }
    squares
}

//integer_points_inside_sphere_slow
//Purpose:
//    Returns all integer points (x,y,z) that are within the given radius from the center.
//Pre-conditions:
//    radius is non-negative.
//Notes:
//    This does it the slow way and check every integer triple would could be in range and uses the square root
//    Meant to validate integer_points_inside_sphere
fn integer_points_inside_sphere_slow(center: Point3D, radius: isize) -> Vec<Point3D>{
    let mut cubes: Vec<Point3D> = Vec::new();
    for x_cord in -radius .. radius +1 {
        for y_cord in -radius .. radius +1 {
            for z_cord in -radius .. radius +1 {
                if ((x_cord*x_cord) + (y_cord*y_cord) + (z_cord*z_cord)) <= radius*radius{
                    cubes.push(Point3D{ x: x_cord + center.x, y: y_cord + center.y, z: z_cord + center.z,});
                }
            }
        }
    }
    cubes
}

//integer_points_inside_sphere_helper
//Purpose:
//    Returns all integer points (x,y) that are within the square root of radius_squared from the center.
//Pre-conditions:
//    radius_squared is non-negaitive.
//Notes:
//    This is a helper function for integer_points_inside_l_2_sphere
fn integer_points_inside_sphere_helper(center: Point2D, radius_squared: isize) -> Vec<Point2D>{
    let mut squares: Vec<Point2D> = Vec::new();
    let radius: isize = (radius_squared as f32).sqrt() as isize;
    let farthest_digonal_integer = ((radius_squared as f32)/2.0).sqrt() as isize;
    for x_cords in -farthest_digonal_integer .. farthest_digonal_integer +1 {
        for y_cords in -farthest_digonal_integer .. farthest_digonal_integer +1 {
            squares.push(Point2D{ x: x_cords + center.x, y: y_cords + center.y,});
        }
    }
    for x_cord in farthest_digonal_integer + 1 .. radius + 1 {
        squares.push(Point2D{ x: x_cord + center.x, y: center.y,});
        squares.push(Point2D{ x: -x_cord + center.x, y: center.y,});
        squares.push(Point2D{ x: center.x, y: x_cord + center.y,});
        squares.push(Point2D{ x: center.x, y: -x_cord + center.y,});
    }
    for x_cord in farthest_digonal_integer + 1 .. radius + 1 {
        for y_cord in 1 .. (((radius_squared - x_cord*x_cord) as f32).sqrt() as isize) + 1 {
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

//integer_points_inside_sphere
//Purpose:
//    Returns all integer points (x,y,z) that are within the given radius from the center.
//Pre-conditions:
//    radius is non-negitive
fn integer_points_inside_sphere(center: Point3D, radius: isize) -> Vec<Point3D>{
    let mut cubes: Vec<Point3D> = Vec::new();
    let mut squares: Vec<Point2D>;
    squares = integer_points_inside_sphere_helper(Point2D{ x: center.x, y: center.y,}, radius*radius);
    for square in squares {
        cubes.push(Point3D{x: square.x, y: square.y, z: center.z});
    }
    for z_cord in 1 .. radius+1 {
        squares = integer_points_inside_sphere_helper(Point2D{ x: center.x, y: center.y,}, radius*radius - z_cord*z_cord);
       for square in squares {
            cubes.push(Point3D{x: square.x, y: square.y, z: center.z + z_cord});
            cubes.push(Point3D{x: square.x, y: square.y, z: center.z - z_cord});
        }
    }
    cubes
}

//integer_points_inside_dimond_3d
//Purpose:
//    Returns all integers points (x,y,z) such that |x -center.x| + |y - center.y| + |z - center.z| <= radius.
//    Equivalent to returing all integer points (x,y,z) that are within the given radius (using the L 1 metric) from the center.
//Pre-conditions:
//    radius is non-negative.
//Notes:
//    Since this is the L 1 metric only loops are needed. This is also know as the taxi-cab metric.
fn integer_points_inside_dimond_3d(center: Point3D, radius: isize) -> Vec<Point3D>{
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

//integer_points_inside_square
//Purpose:
//    Returns all integers points (x,y,z) such that max(|x -center.x|,|y - center.y|,|z - center.z|) <= radius.
//    Equivalent to returing all integer points that are within the given radius (using the L infinity metric) from the center.
//Pre-conditions:
//    radius is non-negative.
//Notes:
//    Since this is the L infinity metric only loops are needed.
fn integer_points_inside_l_infinity_sphere(center: Point3D, radius: isize) -> Vec<Point3D>{
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

fn main() {
    let squares = integer_cells_on_line_segment_2d(Point2D{ x: 0, y: 0,},Point2D{ x: 2, y: 1,});
    for square in squares {
        println!("{:?}",square);
    }
    println!("Done");
}

fn sort<A, T>(mut array: A) -> A
where
    A: AsMut<[T]>,
    T: Ord,
{
    let slice = array.as_mut();
    slice.sort();

    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn points_in_sphere(){
        assert_eq!(sort(integer_points_inside_sphere_slow(Point3D{ x: 0, y: 0, z: 0,},0)),
                   sort(integer_points_inside_sphere(Point3D{ x: 0, y: 0, z: 0,},0)));
        assert_eq!(sort(integer_points_inside_sphere_slow(Point3D{ x: 0, y: 0, z: 0,},1)),
                   sort(integer_points_inside_sphere(Point3D{ x: 0, y: 0, z: 0,},1)));
        assert_eq!(sort(integer_points_inside_sphere_slow(Point3D{ x: 0, y: 0, z: 0,},3)),
                   sort(integer_points_inside_sphere(Point3D{ x: 0, y: 0, z: 0,},3)));
        assert_eq!(sort(integer_points_inside_sphere_slow(Point3D{ x: 0, y: 0, z: 0,},10)),
                   sort(integer_points_inside_sphere(Point3D{ x: 0, y: 0, z: 0,},10)));
        assert_eq!(sort(integer_points_inside_sphere_slow(Point3D{ x: 0, y: 0, z: 0,},100)),
                   sort(integer_points_inside_sphere(Point3D{ x: 0, y: 0, z: 0,},100)));
    }

    #[test]
    fn points_in_circle(){
        assert_eq!(sort(integer_points_inside_circle_slow(Point2D{ x: 0, y: 0,},0)),
                   sort(integer_points_inside_circle(Point2D{ x: 0, y: 0,},0)));
        assert_eq!(sort(integer_points_inside_circle_slow(Point2D{ x: 0, y: 0,},1)),
                   sort(integer_points_inside_circle(Point2D{ x: 0, y: 0,},1)));
        assert_eq!(sort(integer_points_inside_circle_slow(Point2D{ x: 0, y: 0,},10)),
                   sort(integer_points_inside_circle(Point2D{ x: 0, y: 0,},10)));
        assert_eq!(sort(integer_points_inside_circle_slow(Point2D{ x: 0, y: 0,},100)),
                   sort(integer_points_inside_circle(Point2D{ x: 0, y: 0,},100)));
    }

    #[test]
    fn line_2d_endpoints_order(){
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: -1, y: 0,},Point2D{ x: 1, y: 0,})),
                   sort(integer_points_on_line_segment_2d(Point2D{ x: 1, y: 0,},Point2D{ x: -1, y: 0,})));
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: 0, y: -1,},Point2D{ x: 0, y: 1,})),
                   sort(integer_points_on_line_segment_2d(Point2D{ x: 0, y: 1,},Point2D{ x: 0, y: -1,})));
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: 10,})),
                   sort(integer_points_on_line_segment_2d(Point2D{ x: 5, y: 10,},Point2D{ x: 1, y: 2,})));
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: -6,})),
                   sort(integer_points_on_line_segment_2d(Point2D{ x: 5, y: -6,},Point2D{ x: 1, y: 2,})));
    }

    #[test]
    fn line_2d_correct_points(){
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: -1, y: 0,},Point2D{ x: 1, y: 0,})),
                   sort([Point2D{ x: 1, y: 0,},Point2D{ x: 0, y: 0,},Point2D{ x: -1, y: 0,}]));
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: -1, y: 1,},Point2D{ x: 1, y: 1,})),
                   sort([Point2D{ x: 1, y: 1,},Point2D{ x: 0, y: 1,},Point2D{ x: -1, y: 1,}]));
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: 10,})),
                   sort([Point2D{ x: 1, y: 2,},Point2D{ x: 2, y: 4,},
                        Point2D{ x: 3, y: 6,},Point2D{ x: 4, y: 8,},Point2D{ x: 5, y: 10,}]));
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: -6,})),
                   sort([Point2D{ x: 1, y: 2,},Point2D{ x: 2, y: 0,},
                        Point2D{ x: 3, y: -2,},Point2D{ x: 4, y: -4,},Point2D{ x: 5, y: -6,}]));
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: 0, y: 0,},Point2D{ x: 3, y: 5,})),
                   sort([Point2D{ x: 0, y: 0,},Point2D{ x: 3, y: 5,}]));
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: 0, y: 0,},Point2D{ x: 4, y: 2,})),
                   sort([Point2D{ x: 0, y: 0,},Point2D{ x: 2, y: 1,},Point2D{ x: 4, y: 2,}]));
        assert_eq!(sort(integer_points_on_line_segment_2d(Point2D{ x: -3, y: 4,},Point2D{ x: -1, y: 6,})),
                   sort([Point2D{ x: -3, y: 4,},Point2D{ x: -2, y: 5,},Point2D{ x: -1, y: 6,}]));
    }

    #[test]
    fn l_infinity_line_2d_order(){
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: -1, y: 0,},Point2D{ x: 1, y: 0,})),
                   sort(integer_cells_on_line_segment_2d(Point2D{ x: 1, y: 0,},Point2D{ x: -1, y: 0,})));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: 0, y: -1,},Point2D{ x: 0, y: 1,})),
                   sort(integer_cells_on_line_segment_2d(Point2D{ x: 0, y: 1,},Point2D{ x: 0, y: -1,})));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: 0, y: 0,},Point2D{ x: 7, y: 7,})),
                   sort(integer_cells_on_line_segment_2d(Point2D{ x: 7, y: 7,},Point2D{ x: 0, y: 0,})));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: -1, y: -2,},Point2D{ x: 4, y: 3,})),
                   sort(integer_cells_on_line_segment_2d(Point2D{ x: 4, y: 3,},Point2D{ x: -1, y: -2,})));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: 10,})),
                   sort(integer_cells_on_line_segment_2d(Point2D{ x: 5, y: 10,},Point2D{ x: 1, y: 2,})));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: -6,})),
                   sort(integer_cells_on_line_segment_2d(Point2D{ x: 5, y: -6,},Point2D{ x: 1, y: 2,})));
    }

    #[test]
    fn l_infinity_line_2d_points(){
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: -1, y: 0,},Point2D{ x: 1, y: 0,})),
                   sort([Point2D{ x: -1, y: 0,},Point2D{ x: 0, y: 0,},Point2D{ x: 1, y: 0,}]));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: 0, y: 1,},Point2D{ x: 1, y: 0,})),
                   sort(vec![Point2D{ x: 0, y: 1,},Point2D{ x: 1, y: 0,}]));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: 0, y: 0,},Point2D{ x: 2, y: 1,})),
                   sort([Point2D{ x: 0, y: 0,},Point2D{ x: 1, y: 0,},
                        Point2D{ x: 1, y: 1,},Point2D{ x: 2, y: 1,}]));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: -2, y: 0,},Point2D{ x: 1, y: 4,})),
                   sort([Point2D{ x: -2, y: 0,},Point2D{ x: -2, y: 1,},
                        Point2D{ x: -1, y: 1,},Point2D{ x: -1, y: 2,},
                        Point2D{ x: 0, y: 2,},Point2D{ x: 0, y: 3,},
                        Point2D{ x: 1, y: 3,},Point2D{ x: 1, y: 4,}]));
    }

    #[test]
    fn l_infinity_line_rational_2d_order(){
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: -1, y: 0,},Point2D{ x: 1, y: 0,},Rational::new_rational(1,4))),
                   sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 1, y: 0,},Point2D{ x: -1, y: 0,},Rational::new_rational(1,4))));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 0, y: -1,},Point2D{ x: 0, y: 1,},Rational::new_rational(1,4))),
                   sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 0, y: 1,},Point2D{ x: 0, y: -1,},Rational::new_rational(1,4))));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 0, y: 0,},Point2D{ x: 7, y: 7,},Rational::new_rational(1,4))),
                   sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 7, y: 7,},Point2D{ x: 0, y: 0,},Rational::new_rational(1,4))));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: -1, y: -2,},Point2D{ x: 4, y: 3,},Rational::new_rational(1,4))),
                   sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 4, y: 3,},Point2D{ x: -1, y: -2,},Rational::new_rational(1,4))));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: 10,},Rational::new_rational(1,4))),
                   sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 5, y: 10,},Point2D{ x: 1, y: 2,},Rational::new_rational(1,4))));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: -6,},Rational::new_rational(1,4))),
                   sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 5, y: -6,},Point2D{ x: 1, y: 2,},Rational::new_rational(1,4))));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: 10,},Rational::new_rational(1,6))),
                   sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 5, y: 10,},Point2D{ x: 1, y: 2,},Rational::new_rational(1,6))));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: -6,},Rational::new_rational(1,6))),
                   sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 5, y: -6,},Point2D{ x: 1, y: 2,},Rational::new_rational(1,6))));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: 10,},Rational::new_rational(2,6))),
                   sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 5, y: 10,},Point2D{ x: 1, y: 2,},Rational::new_rational(2,6))));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: -6,},Rational::new_rational(2,6))),
                   sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 5, y: -6,},Point2D{ x: 1, y: 2,},Rational::new_rational(2,6))));
    }

    #[test]
    fn l_infinity_line_rational_2d_points(){
        assert_ne!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 0, y: 0,},Point2D{ x: 2, y: 1,},Rational::new_rational(1,4))),
                   sort(vec![Point2D{ x: 0, y: 0,},Point2D{ x: 1, y: 0,},Point2D{ x: 1, y: 1,},Point2D{ x: 2, y: 1,}]));

        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: -1, y: 0,},Point2D{ x: 1, y: 0,},Rational::new_rational(1,4))),
                   sort(vec![Point2D{ x: -1, y: 0,},Point2D{ x: 0, y: 0,},Point2D{ x: 1, y: 0,}]));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 0, y: 0,},Point2D{ x: 2, y: 1,},Rational::new_rational(1,4))),
                   sort(vec![Point2D{ x: 0, y: 0,},Point2D{ x: 2, y: 1,}]));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 0, y: 0,},Point2D{ x: 2, y: 1,},Rational::new_rational(1,3))),
                   sort(vec![Point2D{ x: 0, y: 0,},Point2D{ x: 2, y: 1,}]));
        assert_eq!(sort(integer_cells_on_line_segment_2d_rational_scale(Point2D{ x: 0, y: 0,},Point2D{ x: 2, y: 1,},Rational::new_rational(13,33))),
                   sort(vec![Point2D{ x: 0, y: 0,},Point2D{ x: 1, y: 0,},Point2D{ x: 1, y: 1,},Point2D{ x: 2, y: 1,}]));
    }

    #[test]
    fn l_infinity_line_rational_endpoints_integer_check(){
        let zero = Rational::new_rational(0,1);
        let one = Rational::new_rational(1,1);
        let two = one + one;
        let three = one + two;
        let four = one + three;
        let five = one + four;
        let six = one + five;
        let seven = one + six;
        let ten = five + five;
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: -1, y: 0,},Point2D{ x: 1, y: 0,})),
                   sort(l_infinity_line_rational_endpoints(RationalPoint2D{ x: -one, y: zero,},RationalPoint2D{ x: one, y: zero,})));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: 0, y: -1,},Point2D{ x: 0, y: 1,})),
                   sort(l_infinity_line_rational_endpoints(RationalPoint2D{ x: zero, y: -one,},RationalPoint2D{ x: zero, y: one,})));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: 0, y: 0,},Point2D{ x: 7, y: 7,})),
                   sort(l_infinity_line_rational_endpoints(RationalPoint2D{ x: zero, y: zero,},RationalPoint2D{ x: seven, y: seven,})));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: -1, y: -2,},Point2D{ x: 4, y: 3,})),
                   sort(l_infinity_line_rational_endpoints(RationalPoint2D{ x: -one, y: -two,},RationalPoint2D{ x: four, y: three,})));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: 10,})),
                   sort(l_infinity_line_rational_endpoints(RationalPoint2D{ x: one, y: two,},RationalPoint2D{ x: five, y: ten,})));
        assert_eq!(sort(integer_cells_on_line_segment_2d(Point2D{ x: 1, y: 2,},Point2D{ x: 5, y: -6,})),
                   sort(l_infinity_line_rational_endpoints(RationalPoint2D{ x: one, y: two,},RationalPoint2D{ x: five, y: -six,})));
    }

    #[test]
    fn line_3d_endpoints_order(){
        assert_eq!(sort(integer_points_on_line_segment_3d(Point3D{ x: -1, y: 0, z: 0,},Point3D{ x: 1, y: 0, z: 0,})),
                   sort(integer_points_on_line_segment_3d(Point3D{ x: 1, y: 0, z: 0,},Point3D{ x: -1, y: 0, z: 0,})));
        assert_eq!(sort(integer_points_on_line_segment_3d(Point3D{ x: 0, y: 1, z: 0,},Point3D{ x: 0, y: -1, z: 0,})),
                   sort(integer_points_on_line_segment_3d(Point3D{ x: 0, y: -1, z: 0,},Point3D{ x: 0, y: 1, z: 0,})));
        assert_eq!(sort(integer_points_on_line_segment_3d(Point3D{ x: 0, y: 0, z: 1,},Point3D{ x: 0, y: 0, z: -1,})),
                   sort(integer_points_on_line_segment_3d(Point3D{ x: 0, y: 0, z: -1,},Point3D{ x: 0, y: 0, z: 1,})));
        assert_eq!(sort(integer_points_on_line_segment_3d(Point3D{ x: 0, y: 0, z: 0,},Point3D{ x: 3, y: 6, z: 9,})),
                   sort(integer_points_on_line_segment_3d(Point3D{ x: 3, y: 6, z: 9,},Point3D{ x: 0, y: 0, z: 0,})));
        assert_eq!(sort(integer_points_on_line_segment_3d(Point3D{ x: 0, y: 0, z: 0,},Point3D{ x: 3, y: 6, z: 9,})),
                   sort(integer_points_on_line_segment_3d(Point3D{ x: 3, y: 6, z: 9,},Point3D{ x: 0, y: 0, z: 0,})));
        assert_eq!(sort(integer_points_on_line_segment_3d(Point3D{ x: -1, y: -2, z: -3,},Point3D{ x: 4, y: 23, z: 2,})),
                   sort(integer_points_on_line_segment_3d(Point3D{ x: 4, y: 23, z: 2,},Point3D{ x: -1, y: -2, z: -3,})));
        assert_eq!(sort(integer_points_on_line_segment_3d(Point3D{ x: 3, y: 6, z: 0,},Point3D{ x: 5, y: -8, z: 14,})),
                   sort(integer_points_on_line_segment_3d(Point3D{ x: 5, y: -8, z: 14,},Point3D{ x: 3, y: 6, z: 0,})));
    }

    #[test]
    fn line_3d_correct_points(){
        assert_eq!(sort(integer_points_on_line_segment_3d(Point3D{ x: -1, y: 0, z: 0,},Point3D{ x: 1, y: 0, z: 0,})),
                   sort(vec![Point3D{ x: -1, y: 0, z: 0,},Point3D{ x: 0, y: 0, z: 0,},Point3D{ x: 1, y: 0, z: 0,}]));
        assert_eq!(sort(integer_points_on_line_segment_3d(Point3D{ x: 3, y: 6, z: 9,},Point3D{ x: 0, y: 0, z: 0,})),
                   sort(vec![Point3D{ x: 3, y: 6, z: 9,},Point3D{ x: 2, y: 4, z: 6,},Point3D{ x: 1, y: 2, z: 3,},Point3D{ x: 0, y: 0, z: 0,}]));
        assert_eq!(sort(integer_points_on_line_segment_3d(Point3D{ x: -1, y: -2, z: -3,},Point3D{ x: 4, y: 23, z: 2,})),
                   sort(vec![Point3D{ x: -1, y: -2, z: -3,},Point3D{ x: 0, y: 3, z: -2,},Point3D{ x: 1, y: 8, z: -1,},Point3D{ x: 2, y: 13, z: 0,},
                        Point3D{ x: 3, y: 18, z: 1,},Point3D{ x: 4, y: 23, z: 2,}]));
    }
}
