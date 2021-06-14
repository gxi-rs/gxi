use crate::*;

create_web_container!(Embed);
impl_web_container!(Embed "embed");

impl Embed {
    generate_pub_attr!(height);
    generate_pub_attr!(src);
    generate_pub_attr!(r#type);
    generate_pub_attr!(width);
}
