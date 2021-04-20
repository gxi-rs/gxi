use crate::*;

create_web_container!(Textarea);
impl_web_container!(Textarea "textarea");

impl Textarea {
    generate_pub_attr!(autocapitalize);
    generate_pub_attr!(autocomplete);
    generate_pub_attr!(autocorrect);
    generate_pub_bool_attr!(autofocus);
    generate_pub_attr!(cols u32);
    generate_pub_bool_attr!(disabled);
    generate_pub_attr!(form);
    generate_pub_attr!(maxlength u32);
    generate_pub_attr!(minlength u32);
    generate_pub_attr!(name);
    generate_pub_attr!(placeholder);
    generate_pub_bool_attr!(readonly);
    generate_pub_bool_attr!(readonly);
    generate_pub_bool_attr!(required);
    generate_pub_attr!(rows u32);
    generate_pub_attr!(spellcheck);
    generate_pub_attr!(wrap);
}
