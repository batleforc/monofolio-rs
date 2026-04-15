use icons::common::icon_type::IconType;
use icons::leptos::icon_component::LeptosIcon;
use leptos::prelude::*;
use tw_merge::IntoTailwindClass;

use crate::components::ui::{Card, SectionInner, SectionTitle};
use crate::components::ui::{ButtonClass, ButtonSize, ButtonVariant};
use crate::i18n::{use_language, Language};
use crate::pages::home::HomeData;

#[cfg_attr(feature = "ssr", allow(dead_code))]
async fn load_home_data() -> Option<HomeData> {
    #[cfg(not(feature = "ssr"))]
    {
        let resp = gloo_net::http::Request::get("/api/v1/home")
            .send()
            .await
            .ok()?;
        let text = resp.text().await.ok()?;
        serde_json::from_str::<HomeData>(&text).ok()
    }
    #[cfg(feature = "ssr")]
    {
        None
    }
}

#[component]
pub fn ContactPage() -> impl IntoView {
    let lang = use_language();
    let home_data: RwSignal<Option<HomeData>> = RwSignal::new(None);
    let loading = RwSignal::new(true);
    let fetch_error = RwSignal::new(false);

    let do_fetch = move || {
        loading.set(true);
        fetch_error.set(false);
        #[cfg(not(feature = "ssr"))]
        {
            wasm_bindgen_futures::spawn_local(async move {
                match load_home_data().await {
                    Some(data) => {
                        home_data.set(Some(data));
                        fetch_error.set(false);
                    }
                    None => fetch_error.set(true),
                }
                loading.set(false);
            });
        }
    };

    #[cfg(not(feature = "ssr"))]
    do_fetch();

    let retry_btn_class = ButtonClass {
        variant: ButtonVariant::Primary,
        size: ButtonSize::Default,
    }
    .to_class();

    let page_title = move || match lang.get() {
        Language::Fr => "Contact",
        Language::En => "Contact",
    };

    let intro = move || match lang.get() {
        Language::Fr => "Dispo pour échanger sur des sujets Dev, Ops, architecture et plateformes.",
        Language::En => "Available to discuss Dev, Ops, architecture and platform topics.",
    };

    let availability_label = move || match lang.get() {
        Language::Fr => "Disponibilité",
        Language::En => "Availability",
    };


    let contact_links_title = move || match lang.get() {
        Language::Fr => "Liens",
        Language::En => "Links",
    };

    let current_work_title = move || match lang.get() {
        Language::Fr => "Poste actuel",
        Language::En => "Current work",
    };

    let location_title = move || match lang.get() {
        Language::Fr => "Location",
        Language::En => "Location",
    };

    let email_title = move || match lang.get() {
        Language::Fr => "Email",
        Language::En => "Email",
    };

    view! {
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <SectionInner>
                <SectionTitle>{page_title}</SectionTitle>
                <p class="text-muted-foreground mb-8 max-w-2xl">{intro}</p>

                <Show when=move || loading.get()>
                    <div class="min-h-40 flex items-center text-muted-foreground">
                        "Loading contact…"
                    </div>
                </Show>

                <Show when=move || fetch_error.get() && !loading.get()>
                    <div class="min-h-40 flex flex-col gap-4 items-start justify-center">
                        <p class="text-muted-foreground">"Unable to load contact data."</p>
                        <button class=retry_btn_class.clone() on:click=move |_| do_fetch()>
                            "Retry"
                        </button>
                    </div>
                </Show>

                <Show when=move || {
                    !loading.get() && !fetch_error.get()
                }>
                    {move || {
                        home_data
                            .get()
                            .map(|data| {
                                let availability = match lang.get() {
                                    Language::Fr => {
                                        data.contact_availability
                                            .clone()
                                            .unwrap_or_else(|| {
                                                "Ouvert à des discussions techniques et collaborations"
                                                    .to_string()
                                            })
                                    }
                                    Language::En => {
                                        data.contact_availability_en
                                            .clone()
                                            .or(data.contact_availability.clone())
                                            .unwrap_or_else(|| {
                                                "Open to technical discussions and collaborations"
                                                    .to_string()
                                            })
                                    }
                                };
                                let location = data
                                    .contact_location
                                    .clone()
                                    .unwrap_or_else(|| "Niort, France".to_string());
                                let current_work = data
                                    .current_work
                                    .clone()
                                    .unwrap_or_else(|| {
                                        "Ingénieur Socle de fabrication / Couche d'échange"
                                            .to_string()
                                    });
                                let email = data
                                    .contact_email
                                    .clone()
                                    .unwrap_or_else(|| "contact@maxime-leriche.dev".to_string());
                                let mailto = format!("mailto:{}", email);

                                view! {
                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
                                        <Card class="p-5">
                                            <div class="flex items-start gap-3">
                                                <div class="w-9 h-9 rounded border border-primary/40 flex items-center justify-center text-primary">
                                                    <LeptosIcon icon=IconType::Clock class="w-4 h-4" />
                                                </div>
                                                <div>
                                                    <h3 class="font-semibold mb-1">{availability_label}</h3>
                                                    <p class="text-sm text-muted-foreground">{availability}</p>
                                                </div>
                                            </div>
                                        </Card>

                                        <Card class="p-5">
                                            <div class="flex items-start gap-3">
                                                <div class="w-9 h-9 rounded border border-primary/40 flex items-center justify-center text-primary">
                                                    <LeptosIcon icon=IconType::Briefcase class="w-4 h-4" />
                                                </div>
                                                <div>
                                                    <h3 class="font-semibold mb-1">{current_work_title}</h3>
                                                    <p class="text-sm text-muted-foreground">{current_work}</p>
                                                </div>
                                            </div>
                                        </Card>

                                        <Card class="p-5">
                                            <div class="flex items-start gap-3">
                                                <div class="w-9 h-9 rounded border border-primary/40 flex items-center justify-center text-primary">
                                                    <LeptosIcon icon=IconType::ExternalLink class="w-4 h-4" />
                                                </div>
                                                <div>
                                                    <h3 class="font-semibold mb-1">{contact_links_title}</h3>
                                                    <div class="text-sm text-muted-foreground flex flex-col gap-1">
                                                        {data
                                                            .url
                                                            .iter()
                                                            .map(|link| {
                                                                view! {
                                                                    <a
                                                                        class="hover:text-foreground transition-colors"
                                                                        href=link.url.clone()
                                                                        target="_blank"
                                                                        rel="noopener noreferrer"
                                                                    >
                                                                        {link.name.clone()}
                                                                    </a>
                                                                }
                                                            })
                                                            .collect_view()}
                                                    </div>
                                                </div>
                                            </div>
                                        </Card>

                                        <Card class="p-5">
                                            <div class="flex items-start gap-3">
                                                <div class="w-9 h-9 rounded border border-primary/40 flex items-center justify-center text-primary">
                                                    <LeptosIcon icon=IconType::MapPin class="w-4 h-4" />
                                                </div>
                                                <div>
                                                    <h3 class="font-semibold mb-1">{location_title}</h3>
                                                    <p class="text-sm text-muted-foreground">{location}</p>
                                                </div>
                                            </div>
                                        </Card>

                                        <Card class="p-5 md:col-span-2">
                                            <div class="flex items-start gap-3">
                                                <div class="w-9 h-9 rounded border border-primary/40 flex items-center justify-center text-primary">
                                                    <LeptosIcon icon=IconType::Mail class="w-4 h-4" />
                                                </div>
                                                <div>
                                                    <h3 class="font-semibold mb-1">{email_title}</h3>
                                                    <a
                                                        class="text-sm text-muted-foreground hover:text-foreground transition-colors"
                                                        href=mailto.clone()
                                                    >
                                                        {email.clone()}
                                                    </a>
                                                </div>
                                            </div>
                                        </Card>
                                    </div>
                                }
                            })
                    }}
                </Show>
            </SectionInner>
        </section>
    }
}
