use crate::*;
create_web_container!(Legend);
impl_web_container!(Legend "legend");
impl Legend {
    generate_pub_attr!(datetime);
}
