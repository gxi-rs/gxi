use crate::*;

create_web_container!(Audio);
impl_web_container!(Audio "audio");

impl Audio {
    generate_pub_bool_attr!(autoplay);
    generate_pub_attr!(controls);
    generate_pub_attr!(crossorigin);
    generate_pub_attr!(currentTime);
    generate_pub_bool_attr!(disableRemotePlayback);
    generate_pub_bool_attr!(_loop);
    generate_pub_bool_attr!(muted);
    generate_pub_attr!(preload);
    generate_pub_attr!(src);
}
    