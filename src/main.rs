use crate::{app::App, tui::*};
use std::io::Result;
mod app;
mod tui;
const FPS: u64 = 60;
const BASE_SITE: &'static str = "https://mangareader.to";

fn main() -> Result<()> {
    let app = App::new().run(&mut init()?)?;
    restore()
}

// let mut buf = Vec::new();
// {
//     let mut easy = Easy::new();
//     easy.url(format!("{BASE_SITE}/search?keyword=one+piece").as_str())
//         .unwrap();
//     let mut easy = easy.transfer();
//     easy.write_function(|data| {
//         let fragment = Html::parse_fragment(&String::from_utf8(data.to_vec()).unwrap());
//         let selector = Selector::parse("h3").unwrap();
//         buf.extend(fragment.select(&selector).map(|elem| elem.inner_html()));
//         Ok(data.len())
//     })
//     .unwrap();
//     easy.perform().unwrap();
// };
// let mut terminal = init()?;
// let wdgt = Paragraph::new(buf.into_iter().fold(String::new(), |mut acc, cur| {
//     acc += &cur;
//     acc
// }))
// .green();
