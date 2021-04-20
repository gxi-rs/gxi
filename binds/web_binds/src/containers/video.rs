use crate::*;

create_web_container!(Video);
impl_web_container!(Video "video");
impl Video {
    generate_pub_attr!(datetime);
}
    