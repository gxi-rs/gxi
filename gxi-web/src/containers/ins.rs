use crate::*;

create_web_container!(Ins);
impl_web_container!(Ins "ins");

impl Ins {
    generate_pub_attr!(datetime);
    generate_pub_attr!(cite);
}
