use crate::*;

create_web_container!(Embed);
impl_web_container!(Embed "embed");

impl Embed {
    generate_pub_attr!(height u32);
    generate_pub_attr!(src);
    generate_pub_attr!(_type);
    generate_pub_attr!(width u32);
}
