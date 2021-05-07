use crate::*;

create_web_container!(Data);
impl_web_container!(Data "data");

impl Data {
    generate_pub_attr!(value);
}
