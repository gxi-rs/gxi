use crate::*;

create_web_container!(Math);
impl_web_container!(Math "math");
impl Math {
    generate_pub_attr!(datetime);
}
    