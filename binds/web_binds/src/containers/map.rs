use crate::*;

create_web_container!(Map);
impl_web_container!(Map "map");
impl Map {
    generate_pub_attr!(datetime);
}
    