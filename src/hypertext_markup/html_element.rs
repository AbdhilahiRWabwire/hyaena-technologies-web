#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{option::Option, vec::Vec};

use super::{
    html_attributes::HTMLAttribute, html_end_tags::HTMLEndTag,
    html_global_attributes::HTMLGlobalAttribute, html_start_tags::HTMLStartTag,
};

// Hypertext Markup Language Element Definition
pub struct HypertextMarkupElement {
    pub attributes: Option<Vec<HTMLAttribute>>,
    pub child_elements: Option<Vec<&'static HypertextMarkupElement>>,
    pub global_attributes: Option<Vec<HTMLGlobalAttribute>>,
    pub parent_element: Option<&'static HypertextMarkupElement>,
    pub end_tag: Option<HTMLEndTag>,
    pub start_tag: HTMLStartTag,
}
