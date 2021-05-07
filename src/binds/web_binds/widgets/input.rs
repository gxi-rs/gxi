use crate::*;

create_web_widget!(Input);

impl_web_widget!(Input "input");

impl Input {
    generate_pub_attr!(accept);
    generate_pub_attr!(alt);
    generate_pub_attr!(autocomplete);
    generate_pub_bool_attr!(autofocus);
    generate_pub_attr!(capture);
    generate_pub_bool_attr!(checked);
    generate_pub_attr!(dirname);
    generate_pub_bool_attr!(disabled);
    generate_pub_attr!(form);
    generate_pub_attr!(formaction);
    generate_pub_attr!(formenctype);
    generate_pub_attr!(formmethod);
    generate_pub_attr!(formnovalidate);
    generate_pub_attr!(height u32);
    generate_pub_attr!(inputmode);
    generate_pub_attr!(list);
    generate_pub_attr!(max);
    generate_pub_attr!(maxlength u32);
    generate_pub_attr!(min);
    generate_pub_attr!(minlength u32);
    generate_pub_attr!(multiple);
    generate_pub_attr!(name);
    generate_pub_attr!(pattern);
    generate_pub_attr!(placeholder);
    generate_pub_bool_attr!(readonly);
    generate_pub_bool_attr!(required);
    generate_pub_attr!(size);
    generate_pub_attr!(src);
    generate_pub_attr!(step f32);
    generate_pub_attr!(r#type);
    generate_pub_attr!(value);
    generate_pub_attr!(width u32);
}
