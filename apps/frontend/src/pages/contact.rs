use icons::common::icon_type::IconType;
use icons::leptos::icon_component::LeptosIcon;
use leptos::prelude::*;
use tw_merge::IntoTailwindClass;

use crate::components::ui::{ButtonClass, ButtonSize, ButtonVariant};
use crate::components::ui::{Card, PageTitle, SectionInner};
use crate::i18n::{use_language, use_translations, Language};
use crate::pages::home::HomeData;
use crate::seo::StaticPageSeo;
use crate::services::api::fetch_json;

#[cfg_attr(feature = "ssr", allow(dead_code))]
async fn load_home_data() -> Option<HomeData> {
    fetch_json("/api/v1/home").await
}

#[component]
pub fn ContactPage() -> impl IntoView {
    let lang = use_language();
    let t = use_translations();
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
    let copied = RwSignal::new(false);

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
        Language::Fr => "Localisation",
        Language::En => "Location",
    };

    view! {
        <StaticPageSeo
            title="Contact | Maxime Leriche"
            description="Page de contact pour échanger sur des sujets Dev, Ops, architecture et plateformes."
            path="/contact"
        />
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <SectionInner>
                <PageTitle>{page_title}</PageTitle>
                <p class="text-muted-foreground mb-8 max-w-2xl">{intro}</p>

                <Show when=move || loading.get()>
                    <div class="min-h-40 flex items-center text-muted-foreground">
                        {move || t.get().loading}
                    </div>
                </Show>

                <Show when=move || fetch_error.get() && !loading.get()>
                    <div class="min-h-40 flex flex-col gap-4 items-start justify-center">
                        <p class="text-muted-foreground">{move || t.get().error_loading}</p>
                        <button class=retry_btn_class.clone() on:click=move |_| do_fetch()>
                            {move || t.get().error_retry}
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
                                    Language::Fr => data.contact_availability.clone(),
                                    Language::En => {
                                        data.contact_availability_en
                                            .clone()
                                            .or(data.contact_availability.clone())
                                    }
                                };
                                // Only real data: rows without a value are not rendered.
                                let rows = [
                                    (availability_label(), availability),
                                    (current_work_title(), data.current_work.clone()),
                                    (location_title(), data.contact_location.clone()),
                                ]
                                    .into_iter()
                                    .filter_map(|(label, value)| value.map(|v| (label, v)))
                                    .collect::<Vec<_>>();
                                let email = data.contact_email.clone();
                                let primary_btn_class = ButtonClass {
                                    variant: ButtonVariant::Primary,
                                    size: ButtonSize::Default,
                                }
                                .to_class();
                                let outline_btn_class = ButtonClass {
                                    variant: ButtonVariant::Outline,
                                    size: ButtonSize::Default,
                                }
                                .to_class();
                                let links = data.url.clone();

                                view! {
                                    <Card class="p-6 max-w-2xl">
                                        {email
                                            .map(|email| {
                                                let mailto = format!("mailto:{email}");
                                                #[cfg(not(feature = "ssr"))]
                                                let copy_text = email.clone();
                                                view! {
                                                    <div class="flex flex-wrap items-center gap-3 mb-6">
                                                        <a href=mailto class=primary_btn_class.clone()>
                                                            <LeptosIcon icon=IconType::Mail class="w-4 h-4" />
                                                            <span>{email.clone()}</span>
                                                        </a>
                                                        <button
                                                            type="button"
                                                            class=outline_btn_class.clone()
                                                            on:click=move |_| {
                                                                #[cfg(not(feature = "ssr"))]
                                                                {
                                                                    let copy_text = copy_text.clone();
                                                                    wasm_bindgen_futures::spawn_local(async move {
                                                                        let Some(window) = web_sys::window() else {
                                                                            return;
                                                                        };
                                                                        let promise = window
                                                                            .navigator()
                                                                            .clipboard()
                                                                            .write_text(&copy_text);
                                                                        if wasm_bindgen_futures::JsFuture::from(promise)
                                                                            .await
                                                                            .is_ok()
                                                                        {
                                                                            copied.set(true);
                                                                        }
                                                                    });
                                                                }
                                                            }
                                                        >
                                                            <span aria-live="polite">
                                                                {move || match (copied.get(), lang.get()) {
                                                                    (true, Language::Fr) => "Adresse copiée",
                                                                    (false, Language::Fr) => "Copier l'adresse",
                                                                    (true, Language::En) => "Address copied",
                                                                    (false, Language::En) => "Copy address",
                                                                }}
                                                            </span>
                                                        </button>
                                                    </div>
                                                }
                                            })}

                                        <dl class="grid grid-cols-[max-content_1fr] gap-x-6 gap-y-3 text-sm">
                                            {rows
                                                .into_iter()
                                                .map(|(label, value)| {
                                                    view! {
                                                        <dt class="font-mono text-muted-foreground">{label}</dt>
                                                        <dd class="text-foreground">{value}</dd>
                                                    }
                                                })
                                                .collect_view()}
                                            {(!links.is_empty())
                                                .then(|| {
                                                    view! {
                                                        <dt class="font-mono text-muted-foreground">
                                                            {contact_links_title}
                                                        </dt>
                                                        <dd class="flex flex-wrap gap-x-4 gap-y-1">
                                                            {links
                                                                .into_iter()
                                                                .map(|link| {
                                                                    view! {
                                                                        <a
                                                                            class="text-primary hover:text-foreground underline-offset-2 hover:underline"
                                                                            href=link.url
                                                                            target="_blank"
                                                                            rel="noopener noreferrer"
                                                                        >
                                                                            {link.name}
                                                                        </a>
                                                                    }
                                                                })
                                                                .collect_view()}
                                                        </dd>
                                                    }
                                                })}
                                        </dl>
                                    </Card>
                                }
                            })
                    }}
                </Show>
            </SectionInner>
        </section>
    }
}
