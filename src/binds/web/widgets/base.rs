use crate::*;

create_web_widget!(Base);

impl_web_widget!(Base "base");

impl Base {
    generate_pub_attr!(href);
    generate_pub_attr!(target);
}
