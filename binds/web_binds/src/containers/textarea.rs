use crate::*;
create_web_container!(Textarea);
impl_web_container!(Textarea "textarea");
impl Textarea {
    generate_pub_attr!(datetime);
}
