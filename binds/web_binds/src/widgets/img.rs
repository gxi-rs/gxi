use crate::*;

create_web_widget!(Img);

impl_web_widget!(Img "img");

impl Img {
    generate_pub_attr!(alt);
    generate_pub_attr!(crossorigin);
    generate_pub_attr!(http_equiv);
    generate_pub_attr!(name);
}
