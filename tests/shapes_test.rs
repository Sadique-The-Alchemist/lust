use lust::shapes::shapes::Shape;
use lust::shapes::shapes::{self};

#[test]
fn test_area_and_perimeter_rectangle() {
    let rectangle = shapes::Rectangle {
        width: 10.0,
        height: 10.0,
    };
    let area = rectangle.area();
    let perimeter = rectangle.perimeter();
    assert_eq!(
        perimeter, 40.0,
        "Perimeter is the double of sum of width and height"
    );
    assert_eq!(
        area, 100.0,
        "Area should be the product of width and height"
    );
}

#[test]
fn test_area_and_perimeter_circle() {
    let circle = shapes::Circle { radius: 10.0 };
    let area = circle.area();
    let perimeter = circle.perimeter();
    assert_eq!(area, 314.15927, "area is pie r squire");
    assert_eq!(perimeter, 62.831856, "Perimeter is 2r pie");
}
#[test]
fn do_all_in_a_single_test() {
    let rectangle = shapes::Rectangle {
        width: 10.0,
        height: 10.0,
    };
    let circle = shapes::Circle { radius: 10.0 };
    struct AreaTest<'a> {
        name: &'a str,
        shape: &'a dyn Shape,
        has_area: f32,
        description: &'a str,
    }
    let shapes = [
        AreaTest {
            name: "rectangle",
            shape: &rectangle,
            has_area: 100.1,
            description: "Area should be the product of width and height",
        },
        AreaTest {
            name: "circle",
            shape: &circle,
            has_area: 314.15927,
            description: "area is pie x radius pow 2",
        },
    ];

    for area_test in shapes {
        assert_eq!(
            area_test.shape.area(),
            area_test.has_area,
            "{} -> {}",
            area_test.description,
            area_test.name
        );
    }
}
