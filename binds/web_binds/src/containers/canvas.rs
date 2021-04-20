use crate::*;

create_web_container!(Canvas);
impl_web_container!(Canvas "canvas");
impl Canvas {
    generate_pub_attr!(datetime);
}
    