use crate::*;

create_web_container!(Audio);
impl_web_container!(Audio "audio");
impl Audio {
    generate_pub_attr!(datetime);
}
    