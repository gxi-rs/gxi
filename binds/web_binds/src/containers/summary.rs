use crate::*;

create_web_container!(Summary);
impl_web_container!(Summary "summary");
impl Summary {
    generate_pub_attr!(datetime);
}
    