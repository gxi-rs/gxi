use crate::*;

create_web_container!(Embed);
impl_web_container!(Embed "embed");
impl Embed {
    generate_pub_attr!(datetime);
}
    