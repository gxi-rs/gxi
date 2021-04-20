use crate::*;
create_web_container!(Option);
impl_web_container!(Option "option");
impl Option {
    generate_pub_attr!(datetime);
}
