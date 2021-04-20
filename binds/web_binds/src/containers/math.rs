use crate::*;

create_web_container!(Math);
impl_web_container!(Math "math");

impl Math {
    generate_pub_attr!(dir);
    generate_pub_attr!(href);
    generate_pub_attr!(mathbackground);
    generate_pub_attr!(mathcolor);
    generate_pub_attr!(display);
}
    