use crate::*;
create_web_container!(Label);
impl_web_container!(Label "label");
impl Label {
    generate_pub_attr!(datetime);
}
