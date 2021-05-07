use crate::*;

create_web_container!(Track);
impl_web_container!(Track "track");

impl Track {
    generate_pub_attr!(default);
    generate_pub_attr!(kind);
    generate_pub_attr!(label);
    generate_pub_attr!(src);
    generate_pub_attr!(srclang);
}
