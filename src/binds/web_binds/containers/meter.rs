use crate::*;

create_web_container!(Meter);
impl_web_container!(Meter "meter");

impl Meter {
    generate_pub_attr!(value f32);
    generate_pub_attr!(min f32);
    generate_pub_attr!(max f32);
    generate_pub_attr!(low f32);
    generate_pub_attr!(high f32);
    generate_pub_attr!(optimum f32);
    generate_pub_attr!(form);
}
