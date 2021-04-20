use crate::*;
create_web_container!(Optgroup);
impl_web_container!(Optgroup "optgroup");
impl Optgroup {
    generate_pub_attr!(datetime);
}
