use crate::*;

create_web_container!(Dialog);
impl_web_container!(Dialog "dialog");

impl Dialog {
    generate_pub_attr!(datetime);
}
    