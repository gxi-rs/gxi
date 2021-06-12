use crate::*;

create_web_container!(Svg);
impl_web_container!(Svg "svg");

impl Svg {
    generate_pub_attr!(height);
    generate_pub_attr!(preserveAspectRatio);
    generate_pub_attr!(viewBox);
    generate_pub_attr!(width);
    generate_pub_attr!(x);
    generate_pub_attr!(y);
}
