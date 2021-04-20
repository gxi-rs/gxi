use crate::*;

create_web_container!(Area);
impl_web_container!(Area "area");

impl Area {
    generate_pub_attr!(datetime);
}
    