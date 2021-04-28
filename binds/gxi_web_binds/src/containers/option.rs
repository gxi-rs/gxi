use crate::*;

create_web_container!(OpTion);
impl_web_container!(OpTion "option");

impl OpTion {
    generate_pub_bool_attr!(disabled);
    generate_pub_attr!(label);
    generate_pub_bool_attr!(selected);
    generate_pub_attr!(value);
}
