use std::collections::HashMap;

use crate::{components::*, utils::time::today, TABLE_OF_CONTENTS};
use episcopal_api::{
    calendar::{Date, BCP1979_CALENDAR},
    library::{CommonPrayer, Library},
    liturgy::{Content, Document, Version},
};
use futures::StreamExt;
use leptos::*;
use rust_i18n::t;
use serde::Serialize;
use serde_derive::Deserialize;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;

#[derive(Deserialize, Clone)]
pub struct DocumentPageParams {
    category: String,
    slug: String,
    version: Version,
    date: Option<Date>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DocumentPageProps {
    doc: Document,
    base_path: String,
}

pub fn document() -> Page<DocumentPageProps, DocumentPageParams> {
    Page::new("document")
        .head_fn(head)
        .body_fn(body)
        .static_props_fn(get_static_props)
        .build_paths_fn(get_static_paths)
}

pub fn head(_locale: &str, props: &DocumentPageProps) -> View {
    let title = match &props.doc.label {
        Some(label) => format!("{} – {}", label, t!("common_prayer")),
        None => t!("common_prayer"),
    };

    view! {
        <>
            <title>{title}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
        </>
    }
}

pub fn get_static_paths() -> Vec<String> {
    vec![
        "{category}/{slug}/{version}".into(),
        "{category}/{slug}/{version}/{date}".into(),
    ]
}

pub fn get_static_props(
    locale: &str,
    _path: &str,
    params: DocumentPageParams,
) -> DocumentPageProps {
    let doc = TABLE_OF_CONTENTS
        .iter()
        .flat_map(|(category, docs)| {
            docs.iter()
                .map(move |(slug, _, doc)| (category.clone(), slug.clone(), doc.version, doc))
        })
        .find(|(category, slug, version, _)| {
            category == &params.category && slug == &params.slug && version == &params.version
        })
        .map(|(_, _, _, doc)| doc.clone())
        .expect("could not find document");
    let doc = if let Some(date) = params.date {
        if let Content::Liturgy(liturgy) = &doc.content {
            let evening = liturgy.evening;
            let day = BCP1979_CALENDAR.liturgical_day(date, evening);
            let prefs = HashMap::new();
            CommonPrayer::compile(
                doc.clone(),
                &BCP1979_CALENDAR,
                &day,
                &day.observed,
                &prefs,
                &liturgy.preferences,
            )
            .unwrap()
        } else {
            doc
        }
    } else {
        doc
    };

    DocumentPageProps {
        doc,
        base_path: format!(
            "/{}/document/{}/{}/{:#?}",
            locale, params.category, params.slug, params.version
        ),
    }
}

pub fn body(locale: &str, props: &DocumentPageProps) -> View {
    let doc = &props.doc;

    let title = match &doc.label {
        Some(label) => label.clone(),
        None => t!("common_prayer"),
    };

    let date_picker = DatePicker::new(t!("date"), None);
    if !is_server!() {
        let mut date = date_picker.date.stream().skip(1);
        let base_path = props.base_path.clone();
        spawn_local(async move {
            // skip the first value, because the initial value of the input will
            // always be emitted but has already been reflected in the page
            while let Some(date) = date.next().await {
                if let Some(date) = date {
                    location()
                        .set_href(&format!("{}/{}", base_path, date))
                        .unwrap_throw();
                } else {
                    location().set_href(&base_path).unwrap_throw();
                }
            }
        })
    }

    let side_menu = if doc.has_date_condition() {
        side_menu(
            Icon::Calendar,
            view! {
                <section class="preview-menu">
                    <dyn:view view={date_picker.view()}/>
                </section>
            },
        )
    } else {
        View::Empty
    };

    view! {
        <>
            {header_with_side_menu(locale, &title, side_menu)}
            <main>
                <dyn:view view={document_view(locale, doc)}/>
            </main>
        </>
    }
}
