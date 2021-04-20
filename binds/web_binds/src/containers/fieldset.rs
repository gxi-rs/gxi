use crate::*;
create_web_container!(Fieldset);
impl_web_container!(Fieldset "fieldset");
impl Fieldset {
    generate_pub_attr!(datetime);
}
