use crate::*;
create_web_container!(Progress);
impl_web_container!(Progress "progress");
impl Progress {
    generate_pub_attr!(datetime);
}
