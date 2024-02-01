use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/coach")]
    Coach,
    #[at("/caas")]
    CTPOaaS,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Coach => html! { <Coach /> },
        Route::CTPOaaS => html! { <CTPOaaS /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component]
fn NotFound() -> Html {
    html! {
        <main class="min-h-screen flex flex-col items-center">
            <div class="w-4/5 md:w-3/5 flex flex-col items-center">

                <h1 class="mt-10 md:mt-20">
                    <p class="text-4xl text-center">{"Oups, "}<strong>{"Erreur"}</strong>{" !"}</p>
                    <p class="text-lg md:text-xl text-center italic text-maeevick-orange font-bold mt-5">{ "Tu sembles t'être perdu en chemin mais "} <strong class="underline underline-offset-2">{"404"}</strong> {" gobelins t'atendent !" }</p>
                </h1>
                <hr class="w-full my-5"/>
                <img class="" src="/images/goblin.png" alt="A Cartoon Gobelin picture" />
                <div>
                    <span class="text-xl text-center italic">{"Je doute que ce soit ce que tu cherches"}</span>
                    <span>{" 👇"}</span>
                </div>
                <a href="/" class="mt-5 px-5 py-2.5 text-maeevick-orange font-medium rounded-lg text-sm text-center border border-maeevick-orange hover:text-white hover:bg-maeevick-orange focus:ring-4 focus:outline-none focus:ring-maeevick-orange">{"Fuis vers l'accueil"}</a>
                <hr class="w-full my-5"/>
                <footer class="mb-10">
                    <p>{"Made with ❤️ and Powered by 🦀"}<span class="italic">{"(Rust + Yew + Trunk)!"}</span></p>
                </footer>
            </div>
        </main>
    }
}

#[function_component]
fn CTPOaaS() -> Html {
    html! {
        <main class="min-h-screen flex flex-col items-center">
            <div class="w-4/5 md:w-3/5 flex flex-col items-center">
                <img class="mt-20" src="/images/banner.png" alt="the banner with the USP in french : 'ton Product-Market Fit avec Fluidité et Sérénité!', the url : 'www.maeevick.com' and the main services :'Coach SaaS/MicroSaaS, CTPO as a Service and Newsletter'" />

                <h1>
                    <p class="text-4xl text-center mt-10 md:mt-20">{ "ton " } <strong>{ "Product-Market Fit" }</strong>{" avec "}<strong>{"Fluidité"}</strong>{" et "}<strong>{"Sérénité"}</strong>{" !" }</p>
                    <p class="text-lg md:text-xl text-center italic text-maeevick-orange font-bold mt-5">{ "ton CTPO as a Service" }</p>
                </h1>
                <hr class="w-full my-10"/>
                <em class="text-xl text-center">{"A venir..."}</em>
                <hr class="w-full my-10"/>
                <h3 id="caas">
                    <a href="/" class="mt-5 px-5 py-2.5 text-maeevick-orange font-medium rounded-lg text-sm text-center border border-maeevick-orange hover:text-white hover:bg-maeevick-orange focus:ring-4 focus:outline-none focus:ring-maeevick-orange">{"Retour à l'accueil"}</a>
                </h3>
                <hr class="w-full my-10"/>
                <footer class="mb-10">
                    <p>{"Made with ❤️ and Powered by 🦀"}<span class="italic">{"(Rust + Yew + Trunk)!"}</span></p>
                </footer>
            </div>
        </main>
    }
}

#[function_component]
fn Coach() -> Html {
    html! {
        <main class="min-h-screen flex flex-col items-center">
            <div class="w-4/5 md:w-3/5 flex flex-col items-center">
                <img class="mt-20" src="/images/banner.png" alt="the banner with the USP in french : 'ton Product-Market Fit avec Fluidité et Sérénité!', the url : 'www.maeevick.com' and the main services :'Coach SaaS/MicroSaaS, CTPO as a Service and Newsletter'" />

                <h1>
                    <p class="text-4xl text-center mt-10 md:mt-20">{ "ton " } <strong>{ "Product-Market Fit" }</strong>{" avec "}<strong>{"Fluidité"}</strong>{" et "}<strong>{"Sérénité"}</strong>{" !" }</p>
                    <p class="text-lg md:text-xl text-center italic text-maeevick-orange font-bold mt-5">{ "ton Coach SaaS/MicroSaaS" }</p>
                </h1>
                <hr class="w-full my-10"/>
                <em class="text-xl text-center">{"A venir..."}</em>
                <hr class="w-full my-10"/>
                <h3 id="coach">
                    <a href="/" class="mt-5 px-5 py-2.5 text-maeevick-orange font-medium rounded-lg text-sm text-center border border-maeevick-orange hover:text-white hover:bg-maeevick-orange focus:ring-4 focus:outline-none focus:ring-maeevick-orange">{"Retour à l'accueil"}</a>
                </h3>
                <hr class="w-full my-10"/>
                <footer class="mb-10">
                    <p>{"Made with ❤️ and Powered by 🦀"}<span class="italic">{"(Rust + Yew + Trunk)!"}</span></p>
                </footer>
            </div>
        </main>
    }
}

#[function_component]
fn Home() -> Html {
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

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
