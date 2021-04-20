use crate::*;

create_web_widget!(Link);

impl_web_widget!(Link "view");

impl Link {
    generate_pub_attr!(_as);
    generate_pub_attr!(crossorigin);
    generate_pub_attr!(disabled);
    generate_pub_attr!(href);
    generate_pub_attr!(hreflang);
    generate_pub_attr!(imagesizes);
    generate_pub_attr!(imagesrcset);
    generate_pub_attr!(integrity);
    generate_pub_attr!(media);
    generate_pub_attr!(prefetch);
    generate_pub_attr!(referrerpolicy);
    generate_pub_attr!(rel);
    generate_pub_attr!(sizes);
    generate_pub_attr!(title);
    generate_pub_attr!(_type);
}
