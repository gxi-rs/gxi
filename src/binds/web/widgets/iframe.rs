use crate::*;

create_web_container!(Iframe);

impl_web_container!(Iframe "iframe");

impl Iframe {
    generate_pub_attr!(allow);
    generate_pub_attr!(csp);
    generate_pub_attr!(height);
    generate_pub_attr!(loading);
    generate_pub_attr!(name);
    generate_pub_attr!(referrerpolicy);
    generate_pub_attr!(sandbox);
    generate_pub_attr!(src);
    generate_pub_bool_attr!(srcdoc);
    generate_pub_attr!(width);
}
