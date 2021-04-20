use crate::*;
create_web_container!(Select);
impl_web_container!(Select "select");
impl Select {
    generate_pub_attr!(datetime);
}
