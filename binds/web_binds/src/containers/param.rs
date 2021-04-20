use crate::*;

create_web_container!(Param);
impl_web_container!(Param "param");
impl Param {
    generate_pub_attr!(datetime);
}
    