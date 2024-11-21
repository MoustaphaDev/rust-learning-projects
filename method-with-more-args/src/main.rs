use std::cmp::Ordering;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct SideInfo {
    longest_side: u32,
    shortest_side: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        let container_side_info = self.side_info();
        let contained_side_info = other_rect.side_info();

        // This evaluates to `true` if the longest side of the container
        // is greater than the longest side of the contained
        // And the shortest side of the container is greater than
        // the shortest side of the contained
        container_side_info.longest_side > contained_side_info.longest_side
            && container_side_info.shortest_side > contained_side_info.shortest_side
    }

    fn side_info(&self) -> SideInfo {
        match self.width.cmp(&self.height) {
            Ordering::Less => SideInfo {
                longest_side: self.height,
                shortest_side: self.width,
            },
            Ordering::Greater => SideInfo {
                longest_side: self.width,
                shortest_side: self.height,
            },
            Ordering::Equal => SideInfo {
                longest_side: self.width,
                shortest_side: self.height,
            },
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let _square = Rectangle::square(50);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
