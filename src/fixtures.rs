use super::node::Node;
use super::types::ScalarUnit::{Percent, Px, Vh, Vw};
pub fn single_node_fixture() -> Node {
    Node::new().width((100, Px)).height((10, Percent)) // Using percentage for height
}

pub fn vertical_stack_fixture() -> Node {
    let child1 = Node::new().height((50, Px));
    let child2 = Node::new().height((25, Percent)); // Using percentage for height
    let child3 = Node::new().height((5, Vw)); // Using viewport width unit for height

    Node::new()
        .width((200, Px))
        .add_child(child1)
        .add_child(child2)
        .add_child(child3)
}

pub fn horizontal_stack_fixture() -> Node {
    let child1 = Node::new().width((10, Percent)); // Using percentage for width
    let child2 = Node::new().width((100, Px));
    let child3 = Node::new().width((10, Vw)); // Using viewport width unit for width

    Node::new()
        .height((200, Px))
        .add_child(child1)
        .add_child(child2)
        .add_child(child3)
}

pub fn nested_layout_fixture() -> Node {
    let inner_child1 = Node::new().width((50, Px)).height((50, Px));
    let inner_child2 = Node::new().width((20, Percent)).height((5, Vw)); // Mixing percentage and viewport width
    let inner_vertical_stack = Node::new()
        .width((150, Px))
        .add_child(inner_child1)
        .add_child(inner_child2);

    let outer_child = Node::new().width((200, Px)).height((10, Vh)); // Using viewport height unit

    Node::new()
        .add_child(inner_vertical_stack)
        .add_child(outer_child)
}

pub fn vertical_stretch_layout_fixture() -> Node {
    let inner_child1 = Node::new().width((50, Px)).height((50, Px));
    let inner_child2 = Node::new().width((20, Percent)).height((5, Vw)); // Mixing percentage and viewport width
    let inner_vertical_stack = Node::new()
        .width((150, Px))
        .add_child(inner_child1)
        .add_child(inner_child2);

    let outer_child = Node::new().width((200, Px)); // Using viewport height unit

    Node::new()
        .add_child(inner_vertical_stack)
        .add_child(outer_child)
}
