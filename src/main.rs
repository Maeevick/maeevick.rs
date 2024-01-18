use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <main class="min-h-screen flex flex-col items-center">
            <h1 class="text-4xl text-center m-20">{ "Atteint ton " } <strong>{ "Product Market Fit" }</strong>{" avec "}<strong>{"fluidité"}</strong>{" et "}<strong>{"sérénité"}</strong>{" !" }</h1>
            <em class="text-xl text-center my-20">{"A venir... en attendant abonnes-toi à la newsletter 👇"}</em>

            <div>
                <iframe src="https://maeevick.substack.com/embed" width="480" height="320"></iframe>
            </div>

            <footer>{"Made with ❤️ with Rust!"}</footer>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
