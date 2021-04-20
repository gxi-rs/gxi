use crate::*;

create_web_container!(Output);
impl_web_container!(Output "output");

impl Output {
    generate_pub_attr!(_for);
    generate_pub_attr!(form);
    generate_pub_attr!(name);
}
