use crate::*;

create_web_container!(Object);
impl_web_container!(Object "object");
impl Object {
    generate_pub_attr!(data);
    generate_pub_attr!(form);
    generate_pub_attr!(height);
    generate_pub_attr!(name);
    generate_pub_attr!(r#type);
    generate_pub_attr!(usemap);
    generate_pub_attr!(width);
}
