use crate::*;

create_web_container!(Button);
impl_web_container!(Button "button");

impl Button {
    generate_pub_bool_attr!(autofocus);
    generate_pub_bool_attr!(autofocus);
    generate_pub_bool_attr!(disabled);
    generate_pub_attr!(form);
    generate_pub_attr!(formaction);
    generate_pub_attr!(formenctype);
    generate_pub_attr!(formmethod);
    generate_pub_attr!(formnovalidate);
    generate_pub_attr!(formtarget);
    generate_pub_attr!(name);
    generate_pub_attr!(_type);
    generate_pub_attr!(value);
}
