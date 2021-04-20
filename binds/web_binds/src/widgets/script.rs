use crate::*;

create_web_widget!(Script);

impl_web_widget!(Script "script");

impl Script {
    generate_pub_attr!(_async bool ;"async");
    generate_pub_attr!(crossorigin);
    generate_pub_attr!(defer bool);
    generate_pub_attr!(integrity);
    generate_pub_attr!(nomodule bool);
    generate_pub_attr!(nonce);
    generate_pub_attr!(referrerpolicy);
    generate_pub_attr!(src);
    generate_pub_attr!(_type &str; "type");
}
