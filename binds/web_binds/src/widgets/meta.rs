use crate::*;

create_web_widget!(Meta);

impl_web_widget!(Meta "meta");

impl Meta {
    generate_pub_attr!(charset);
    generate_pub_attr!(content);
    generate_pub_attr!(http_equiv);
    generate_pub_attr!(name);
}