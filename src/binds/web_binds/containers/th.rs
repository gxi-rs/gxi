use crate::*;

create_web_container!(Th);
impl_web_container!(Th "th");

impl Th {
    generate_pub_attr!(abbr);
    generate_pub_attr!(colspan u32);
    generate_pub_attr!(headers);
    generate_pub_attr!(rowspan u32);
    generate_pub_attr!(scope);
}
