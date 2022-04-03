//Chapter 5 of the rust book examples
//Struct def used in V3 calculation

//Used to allow our struct to print via {:?} or {:#?} to newline fields
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Using a method syntax instead
impl Rectangle {

    //Associated functions

    // Self must be the first parameter in methods
    // be that mutable, ownership, or immutable
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Methods can have the same name as struct property keys
    fn width(&self) -> bool{
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions that aren't methods
    // Often used as constructors 
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    // Note: these impl blocks could be separate 
    // no need to have them together if it made sense
}

fn main() {
    //  Version  1
    let width1 = 30;
    let height1 = 50;

    println!(
        "V1 - The area of the rectangle is {} square pixels.",
        calculate_area(width1, height1)
    );

    // Version 2 - With tuples
    let rect_v2 = (30, 50);

    println!(
        "V2 - The area of the rectangle is {} square pixels.",
        calculate_area_v2(rect_v2)
    );

    // Version 3 - With structs

    let rect_v3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "V3 - The area of the rectangle is {} square pixels.",
        calculate_area_v3(&rect_v3)
    );

    // Debug printing with {:?} or {:#?}

    // This won't work because our struct doesn't implement std::fmt::Display
    // println!("The struct looks like {}", rect_v3)

    // Single line debug print because we derived debug
    println!("The struct looks like {:?}", rect_v3);

    // Multi pine debug print
    println!("The struct multi line looks like {:#?}", rect_v3);

    // Debug printing can also be done with dbg!

    let scale = 2;
    let rect_v4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect_v4);

    // 5.3 Using method syntax

    let rect_v5 = Rectangle {
        height: 50,
        width: 30
    };

    println!(
        "V5 - The area of the rectangle is {} square pixels.",
        rect_v5.area()
    );

    if rect_v5.width(){
        println!("The rectangle has a nonzero width; it is {}", rect_v5.width);
    }

    let rect_v6 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect_v7 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect_v8 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect_v6 hold rect_v7? {}", rect_v6.can_hold(&rect_v7));
    println!("Can rect_v6 hold rect_v8? {}", rect_v6.can_hold(&rect_v8));

    // Calling associated functions
    let rect_v9 = Rectangle::square(10);

    println!("The square's area is {}", rect_v9.area());
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn calculate_area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
