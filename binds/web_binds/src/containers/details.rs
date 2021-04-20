use crate::*;

create_web_container!(Details);
impl_web_container!(Details "details");

impl Details {
    generate_pub_bool_attr!(open);
}
    