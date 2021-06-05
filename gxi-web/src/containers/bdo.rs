use crate::*;

create_web_container!(Bdo);
impl_web_container!(Bdo "bdo");

impl Bdo {
    generate_pub_attr!(dir);
}
