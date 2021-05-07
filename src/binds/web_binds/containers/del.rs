use crate::*;

create_web_container!(Del);
impl_web_container!(Del "del");

impl Del {
    generate_pub_attr!(cite);
    generate_pub_attr!(datetime);
}
