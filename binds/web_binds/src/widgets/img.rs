use crate::*;

create_web_widget!(Img);

impl_web_widget!(Img "img");

impl Img {
    generate_pub_attr!(alt);
    generate_pub_attr!(crossorigin);
    generate_pub_attr!(decoding);
    generate_pub_attr!(height u32);
    generate_pub_attr!(ismap bool);
    generate_pub_attr!(loading);
    generate_pub_attr!(referrerpolicy);
    generate_pub_attr!(sizes);
    generate_pub_attr!(src);
    generate_pub_attr!(srcset);
    generate_pub_attr!(width u32);
    generate_pub_attr!(usemap);
}
