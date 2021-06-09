use crate::*;

create_web_container!(Select);
impl_web_container!(Select "select");

impl Select {
    generate_pub_attr!(autocomplete);
    generate_pub_bool_attr!(autofocus);
    generate_pub_bool_attr!(disabled);
    generate_pub_attr!(form);
    generate_pub_bool_attr!(multiple);
    generate_pub_attr!(name);
    generate_pub_bool_attr!(required);
    generate_pub_attr!(size);
}
