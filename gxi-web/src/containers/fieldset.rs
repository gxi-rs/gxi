use crate::*;

create_web_container!(Fieldset);
impl_web_container!(Fieldset "fieldset");

impl Fieldset {
    generate_pub_bool_attr!(disabled);
    generate_pub_attr!(form);
    generate_pub_attr!(name);
}
