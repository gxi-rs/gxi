use crate::*;

create_web_container!(Menu);
impl_web_container!(Menu "menu");
impl Menu {
    generate_pub_attr!(datetime);
}
    