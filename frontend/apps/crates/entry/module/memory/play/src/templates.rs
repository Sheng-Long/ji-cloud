use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use utils::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}

const MODULE_PAGE:&'static str = "module-page";
const PLAYER:&'static str = "player";
const CARD:&'static str = "card";
const HOVER_CARD:&'static str = "hover-card";

pub fn module_page_str() -> String {
    TEMPLATES.with(|t| t.cache.render_plain(MODULE_PAGE).to_string())
}

pub fn module_page() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(MODULE_PAGE))
}
pub fn player() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(PLAYER))
}


pub fn card() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CARD))
}

pub fn hover_card() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(HOVER_CARD))
}
pub struct Templates {
    pub cache: TemplateCache<'static>
}

impl fmt::Debug for Templates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        f.debug_list()
            .entries(self.cache.templates.keys())
         .finish()
    }
}
impl Templates {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![
            (MODULE_PAGE, get_template_str(include_str!("../../../../../../../.template_output/_common/module/module-page.html"))),
            (PLAYER, get_template_str(include_str!("../../../../../../../.template_output/module/memory/play/player.html"))),
            (CARD, get_template_str(include_str!("../../../../../../../.template_output/module/memory/play/memory-card.html"))),
            (HOVER_CARD, get_template_str(include_str!("../../../../../../../.template_output/module/memory/play/hover-card.html"))),
        ]);

        Self { cache }
    }

}

//replace {{MEDIA_UI}} in the template string
//this leaks memory - which is okay since templates exist for the lifetime of the app
fn get_template_str(s:&'static str) -> &'static str {
    unsafe {
        Box::leak(SETTINGS.get_unchecked().remote_target.replace_media_ui(s).into_boxed_str())
    }
}
