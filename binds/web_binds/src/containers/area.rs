use crate::*;

create_web_container!(Area);
impl_web_container!(Area "area");

impl Area {
    generate_pub_attr!(alt);
    generate_pub_attr!(coords);
    generate_pub_attr!(download);
    generate_pub_attr!(href);
    generate_pub_attr!(hreflang);
    generate_pub_attr!(ping);
    generate_pub_attr!(referrerpolicy);
    generate_pub_attr!(rel);
    generate_pub_attr!(shape);
    generate_pub_attr!(target);
}
    