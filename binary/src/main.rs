mod demo;

// optional. if not user, defaults to i32
library::meta::api_type!(f32);

fn main() {
    demo::adder_demo();
}
