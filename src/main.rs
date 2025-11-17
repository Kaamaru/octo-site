use dioxus::prelude::*;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const INTER_FONT: &str = "https://fonts.googleapis.com/css2?family=Inter:wght@400;600&display=swap";
const PRECONNECT_GOOGLE_FONTS: &str = "https://fonts.googleapis.com";
const PRECONNECT_GSTATIC: &str = "https://fonts.gstatic.com";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Meta & Stylesheets
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "preconnect", href: PRECONNECT_GOOGLE_FONTS }
        document::Link {
            rel: "preconnect",
            href: PRECONNECT_GSTATIC,
            crossorigin: "anonymous",
        }
        document::Link { rel: "stylesheet", href: INTER_FONT }

        // Page Layout
        div { class: "container",
            header { class: "header",
                h1 { "Hi, me Kaamaru" }
                p {
                    "Lazy ahh rust "
                    s { "developer" }
                    " who chose the "
                    i { "wrong?" }
                    " path instead of the popular JS and can't do shi"
                }
            }
            main { class: "main-content",
                section {
                    h2 { "About Me" }
                    ul {
                        li { "I make games, bad and unfinished ones." }
                        li { "I can't make webapps." }
                    }
                }
                section {
                    h2 { "Projects" }
                    ul {
                        li { "Disappointment and harm to some people." }
                        li {
                            "Rusty Heavy Seabot game "
                            span { style: "color: gray; font-size: 0.9em;", "(currently private)" }
                        }
                    }
                }
                section {
                    h2 { "Contact" }

                    p {
                        // Discord
                        img {
                            src: "https://cdn.jsdelivr.net/gh/twitter/twemoji@latest/assets/svg/1f4ac.svg", // speech balloon icon
                            alt: "Discord",
                            style: "width: 1em; height: 1em; vertical-align: middle; margin-right: 0.4em;",
                        }
                        "Me on Discord: "
                        u {
                            b { "kaamaru" }
                        }
                        br {}

                        span { style: "color: gray; font-size: 0.9em;",
                            "this is literally from discord..."
                        }
                        br {}
                        br {}

                        // GitHub
                        img {
                            src: "https://cdn.jsdelivr.net/gh/simple-icons/simple-icons/icons/github.svg",
                            alt: "GitHub",
                            style: "width: 1em; height: 1em; vertical-align: middle; margin-right: 0.4em; filter: invert(100%);",
                        }
                        a {
                            href: "https://github.com/Kaamaru",
                            target: "_blank",
                            "Kaamaru on GitHub"
                        }
                    }

                    iframe {
                        title: "Discord user embed",
                        width: "340",
                        height: "192",
                        frame_border: "0",
                        src: "https://widgets.vendicated.dev/user?id=1143217634263965777&theme=dark&banner=false&full-banner=false&rounded-corners=true&discord-icon=true&badges=true&guess-nitro=false&foreground-color=%2370f"
                    }
                }
            }
            footer { class: "footer",
                p { "© 2025 Kaamaru. Built with Rust and Dioxus." }
            }
        }
    }
}

