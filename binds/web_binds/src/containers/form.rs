use crate::*;

create_web_container!(Form);

impl_web_container!(Form "form");

impl Form {
    generate_pub_attr!(accept_charset);
    generate_pub_attr!(autocapitalize);
    generate_pub_attr!(autocomplete);
    generate_pub_attr!(name);
    generate_pub_attr!(rel);
    generate_pub_attr!(action);
    generate_pub_attr!(enctype);
    generate_pub_attr!(method);
    generate_pub_attr!(novalidate bool);
    generate_pub_attr!(target);
}