use crate::*;

create_web_container!(Source);
impl_web_container!(Source "source");

impl Source {
    generate_pub_attr!(media);
    generate_pub_attr!(sizes);
    generate_pub_attr!(src);
    generate_pub_attr!(srcset);
    generate_pub_attr!(_type);
}
