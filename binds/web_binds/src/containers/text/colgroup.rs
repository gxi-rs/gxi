
use crate::*;

create_web_container!(Colgroup);

impl_web_container!(Colgroup "colgroup");

impl Colgroup {
    generate_pub_attr!(span u32);
}