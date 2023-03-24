use getrandom;
use yew::{
    html,
    function_component,
    Html,
    use_state
};

#[function_component(App)]
pub fn start() -> Html {
    let counter = use_state(|| 0);
    let number = |num| -> String {
        (0..).map(|_| {
            let mut buf = [0];
            getrandom::getrandom(&mut buf).unwrap();
            buf[0]
        }).filter(|&c| c <= 126 && c >= 33)
            .map(|c| c as char)
            .take(num)
            .collect()
        };

    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html!{
        <>
            <p> {"Current length : "} { *counter } </p>
            <button {onclick}>{ "+1 Length" }</button>
            <p>{ format!("{}", number(*counter)) }</p>
        </>
    }
}