use crate::*;

create_web_container!(Svg);
impl_web_container!(Svg "svg");
impl Svg {
    generate_pub_attr!(datetime);
}
    