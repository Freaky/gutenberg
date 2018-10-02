#[macro_use]
extern crate lazy_static;
extern crate syntect;

use std::path::Path;

use syntect::LoadingError;
use syntect::dumps::from_binary;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Theme};
use syntect::easy::HighlightLines;

// thread_local! {
//     /// A pair of the set and whether extras have been added to it.
//     pub static SYNTAX_SET: RefCell<(SyntaxSet, bool)> = {
//         let ss: SyntaxSet = from_binary(include_bytes!("../../../sublime_syntaxes/newlines.packdump"));
//         RefCell::new((ss, false))
//     };
// }

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = {
        let ss: SyntaxSet = from_binary(include_bytes!("../../../sublime_syntaxes/newlines.packdump"));
        ss
    };
}

lazy_static! {
    pub static ref THEME_SET: ThemeSet = from_binary(include_bytes!("../../../sublime_themes/all.themedump"));
}

pub fn get_highlighter<'a>(theme: &'a Theme, info: &str, base_path: &Path, extra_syntaxes: &[String]) -> Result<HighlightLines<'a>, LoadingError> {
    // SYNTAX_SET.with(|rc| {
    //     let (ss, extras_added) = &mut *rc.borrow_mut();
    //     if !*extras_added {
    //         for dir in extra_syntaxes {
    //             ss.load_syntaxes(base_path.join(dir), true)?;
    //         }
    //         ss.link_syntaxes();
    //         *extras_added = true;
    //     }

        let syntax = info
            .split(' ')
            .next()
            .and_then(|lang| SYNTAX_SET.find_syntax_by_token(lang))
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
        Ok(HighlightLines::new(syntax, theme))
    // })
}
