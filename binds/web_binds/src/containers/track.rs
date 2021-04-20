use crate::*;

create_web_container!(Track);
impl_web_container!(Track "track");
impl Track {
    generate_pub_attr!(datetime);
}
    