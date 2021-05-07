use crate::*;

create_web_container!(Canvas);
impl_web_container!(Canvas "canvas");

impl Canvas {
    generate_pub_attr!(height u32);
    generate_pub_attr!(width u32);
}
