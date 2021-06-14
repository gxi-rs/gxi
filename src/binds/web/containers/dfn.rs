use crate::*;

create_web_container!(Dfn);
impl_web_container!(Dfn "dfn");

impl Dfn {
    generate_pub_attr!(title);
}
