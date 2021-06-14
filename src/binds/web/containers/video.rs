use crate::*;

create_web_container!(Video);
impl_web_container!(Video "video");

impl Video {
    generate_pub_bool_attr!(autoplay);
    generate_pub_bool_attr!(autoPictureInPicture);
    generate_pub_attr!(buffered);
    generate_pub_attr!(controls);
    generate_pub_attr!(controlslist);
    generate_pub_attr!(crossorigin);
    generate_pub_attr!(currentTime);
    generate_pub_bool_attr!(disablePictureInPicture);
    generate_pub_bool_attr!(disableRemotePlayback);
    generate_pub_attr!(height);
    generate_pub_bool_attr!(_loop);
    generate_pub_bool_attr!(muted);
    generate_pub_bool_attr!(playsinline);
    generate_pub_attr!(poster);
    generate_pub_attr!(preload);
    generate_pub_attr!(src);
    generate_pub_attr!(width);
}
