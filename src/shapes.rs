pub mod shapes {
    use std::f32::consts::PI;

    const DOUBLER: f32 = 2.0;

    pub trait Shape {
        fn area(&self) -> f32;
        fn perimeter(&self) -> f32;
    }
    pub struct Rectangle {
        pub width: f32,
        pub height: f32,
    }
    impl Shape for Rectangle {
        fn perimeter(&self) -> f32 {
            return (self.width + self.height) * DOUBLER;
        }
        fn area(&self) -> f32 {
            return self.width * self.height;
        }
    }
    pub struct Circle {
        pub radius: f32,
    }
    impl Shape for Circle {
        fn perimeter(&self) -> f32 {
            return DOUBLER * self.radius * PI;
        }
        fn area(&self) -> f32 {
            return self.radius.powi(2) * PI;
        }
    }
}
