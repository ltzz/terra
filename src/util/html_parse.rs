use scraper::{Html, Selector};


pub fn html_to_text( html_str : &str) -> String {
    let fragment = Html::parse_fragment(html_str);
    
    let css = "p"; // FIXME: pタグ関係なくテキスト化
    let selector = Selector::parse(css).unwrap();


    let elements = fragment.select(&selector);
    let items = elements.map(|e| e.text().next().unwrap_or(""));
    let text = items.collect::<Vec<_>>();

    return text.concat()
}
