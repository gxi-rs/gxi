pub enum Node {
    /// widget which cannot contain other nodes
    Leaf,
    /// widget which can hold other widgets
    Container,
    /// widget which can hold other widgets but can't be added to other nodes
    TopLevelContainer,
}
