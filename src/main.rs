use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <main class="min-h-screen flex flex-col items-center">
            <div class="w-4/5 md:w-3/5 flex flex-col items-center">
                <img class="mt-20" src="/images/banner.png" alt="the banner with the USP in french : 'ton Product-Market Fit avec Fluidité et Sérénité!', the url : 'www.maeevick.com' and the main services :'Coach SaaS/MicroSaaS, CTPO as a Service and Newsletter'" />

                <h1>
                    <p class="text-4xl text-center mt-10 md:mt-20">{ "ton " } <strong>{ "Product-Market Fit" }</strong>{" avec "}<strong>{"Fluidité"}</strong>{" et "}<strong>{"Sérénité"}</strong>{" !" }</p>
                    <p class="text-lg md:text-xl text-center italic text-maeevick-orange font-bold mt-5">{ "Coach SaaS/MicroSaaS · CTPO as a Service · Newsletter" }</p>
                </h1>
                <hr class="w-full my-10"/>
                <em class="text-xl text-center">{"A venir... en attendant abonnes-toi à la newsletter 👇"}</em>
                <hr class="w-full my-10"/>
                <h3 id="newsletter">
                    <iframe src="https://maeevick.substack.com/embed" width="480" height="320"></iframe>
                </h3>
                <hr class="w-full my-10"/>
                <footer class="mb-10">
                    <p>{"Made with ❤️ and Powered by 🦀"}<span class="italic">{"(Rust + Yew + Trunk)!"}</span></p>
                </footer>
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
