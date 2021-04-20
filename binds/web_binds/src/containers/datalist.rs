use crate::*;
create_web_container!(Datalist);
impl_web_container!(Datalist "datalist");
impl Datalist {
    generate_pub_attr!(datetime);
}
