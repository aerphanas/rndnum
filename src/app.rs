use getrandom;
use yew::{html, function_component, Html};

#[function_component(App)]
pub fn start() -> Html {
    let number: String = (0..).map(|_| {
        let mut buf = [0];
        getrandom::getrandom(&mut buf).unwrap();
        buf[0]
    })
    .filter(|&c| c <= 126 && c >= 33)
    .map(|c| c as char)
    .take(1000)
    .collect();
    
    html!{
        <p>{ format!("{}", number) }</p>
    }
}