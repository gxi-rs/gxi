use crate::*;

create_web_container!(Col);

impl_web_container!(Col "col");

impl Col {
    generate_pub_attr!(span u32);
}