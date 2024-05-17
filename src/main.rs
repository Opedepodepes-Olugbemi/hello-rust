use std::f64::consts::PI;

enum Shape<T> {// FIX ME
    Circle(T),
    RightTriangle(T, T),
    Rectangle(T, T),
}

impl<T: Into<f64>> Shape<T> { // FIXED ME
    fn area(self) -> f64 {
        match self {
            Shape::Circle(diameter) => PI * (diameter.into() / 2.0).powf(2.0),
            Shape::RightTriangle(base, height) => 0.5 * base.into() * height.into(),
            Shape::Rectangle(width, length) => width.into() * length.into(),
        }
    }
}

// fixed this main function. Hint: Compile your code
fn main() {
    let base = 24_u8;// FIXED ME
    let height = 24_u8;
    let triangle = Shape::RightTriangle(base, height);
    let triangle_area = triangle.area();

    let width = 12_u8;
    let length = 24_u8;
    let rectangle = Shape::Rectangle(width, length);
    let rectangular_area = rectangle.area();

    let diameter = 45_u8;
    let circle = Shape::Circle(diameter);
    let circle_area = circle.area();

    println!(
        "The area of the triangle with a base of {} and a height of {} is {:.5}",
        base, height, triangle_area
    );
    println!(
        "The area of the rectangle with a width of {} and a length of {} is {:.5}",
        width, length, rectangular_area
    );
    println!(
        "The area of the circle with a diameter of {} is {:.5}",
        diameter, circle_area
    );
}
