use crate::*;

create_web_container!(Picture);
impl_web_container!(Picture "picture");
impl Picture {
    generate_pub_attr!(datetime);
}
