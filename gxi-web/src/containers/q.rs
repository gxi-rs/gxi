use crate::*;

create_web_container!(Q);
impl_web_container!(Q "q");

impl Q {
    generate_pub_attr!(cite);
}
