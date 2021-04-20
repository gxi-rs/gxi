use crate::*;

create_web_container!(Td);
impl_web_container!(Td "td");

impl Td {
    generate_pub_attr!(colspan u32);
    generate_pub_attr!(headers);
    generate_pub_attr!(rowspan u32);
}
    