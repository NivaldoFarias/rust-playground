use std::mem;

#[macro_use]
extern crate prettytable;

use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator},
    Table,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// A Rectangle can be specified by where its top left and bottom right
// corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // (all the type annotations are superfluous)
    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;

    // Create a table
    let mut table = Table::new();

    let format = FormatBuilder::new()
        .column_separator('▏')
        .left_border('▏')
        .right_border('▕')
        .separator(LinePosition::Top, LineSeparator::new('▔', '▔', '◤', '◥'))
        .separator(LinePosition::Bottom, LineSeparator::new('▁', '▁', '◣', '◢'))
        .padding(1, 2)
        .build();

    table.set_format(format);

    table.set_titles(row![FCl -> "Variable",FCc -> "Memory location",FCr -> "Size"]);
    table.add_row(row![
        Fgl -> "point",
        Fmc -> format!("{:p}", &point),
        Fyr -> format!("{} bytes", mem::size_of_val(&point))
    ]);
    table.add_row(row![
        Fgl -> "rectangle",
        Fmc -> format!("{:p}", &rectangle),
        Fyr -> format!("{} bytes", mem::size_of_val(&rectangle))
    ]);
    table.add_row(row![
        Fgl -> "boxed_point",
        Fmc -> format!("{:p}", &boxed_point),
        Fyr -> format!("{} bytes", mem::size_of_val(&boxed_point))
    ]);
    table.add_row(row![
        Fgl -> "boxed_rectangle",
        Fmc -> format!("{:p}", &boxed_rectangle),
        Fyr -> format!("{} bytes", mem::size_of_val(&boxed_rectangle))
    ]);
    table.add_row(row![
        Fgl -> "box_in_a_box",
        Fmc -> format!("{:p}", &box_in_a_box),
        Fyr -> format!("{} bytes", mem::size_of_val(&box_in_a_box))
    ]);
    table.add_row(row![
        Fgl -> "unboxed_point",
        Fmc -> format!("{:p}", &unboxed_point),
        Fyr -> format!("{} bytes", mem::size_of_val(&unboxed_point))
    ]);

    table.printstd();
}
