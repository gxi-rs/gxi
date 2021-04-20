use crate::*;
create_web_container!(Meter);
impl_web_container!(Meter "meter");
impl Meter {
    generate_pub_attr!(datetime);
}
