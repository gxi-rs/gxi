use crate::*;

create_web_container!(Progress);
impl_web_container!(Progress "progress");

impl Progress {
    generate_pub_attr!(max f32);
    generate_pub_attr!(value f32);
}
