use crate::*;

create_web_container!(Portal);
impl_web_container!(Portal "portal");
impl Portal {
    generate_pub_attr!(datetime);
}
    