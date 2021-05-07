use crate::*;

create_web_container!(Time);
impl_web_container!(Time "time");

impl Time {
    generate_pub_attr!(datetime);
}
