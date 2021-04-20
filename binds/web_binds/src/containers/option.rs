use crate::*;

create_web_container!(Option);
impl_web_container!(Option "option");

impl Option {
    generate_pub_bool_attr!(disabled);
    generate_pub_attr!(label);
    generate_pub_bool_attr!(selected);
    generate_pub_attr!(value);
}
