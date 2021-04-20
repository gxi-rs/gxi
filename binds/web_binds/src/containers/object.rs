use crate::*;

create_web_container!(Object);
impl_web_container!(Object "object");
impl Object {
    generate_pub_attr!(datetime);
}
    