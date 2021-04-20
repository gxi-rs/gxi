use crate::*;

create_web_container!(Base);

impl_web_container!(Base "base");

impl Form {
    generate_pub_attr!(href);
    generate_pub_attr!(target);
}