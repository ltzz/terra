use scraper::{Html, Selector};


pub fn html_to_text( html_str : &str) -> String {
    let target_string = format!("<div id=\"__wrapper\">{}</div>", html_str);
    let fragment = Html::parse_fragment(&target_string);
    
    let css = "#__wrapper"; // FIXME: もっといい方法
    let selector = Selector::parse(css).unwrap();


    let elements = fragment.select(&selector);
    let items = elements.map(|e| e.text().next().unwrap_or(""));
    let text = items.collect::<Vec<_>>();

    return text.concat()
}

