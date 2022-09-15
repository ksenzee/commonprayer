use std::fmt::Debug;
use std::rc::Rc;

use calendar::{Date, Feast, LiturgicalDay, LiturgicalDayId, Rank, Weekday};
use itertools::Itertools;
use language::Language;
use leptos::*;
use library::summary;

use crate::header::set_title;
use crate::i18n::{use_i18n, use_locale};
use crate::icon::Icon;
use crate::modal::*;
use crate::time::{get_timezone_offset, today};

#[derive(Params, Debug, Clone, PartialEq, Eq, Default)]
pub struct CalendarDayQuery {
    month: Option<String>,
    calendar: Option<String>,
    blackletter: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarData {
    year: Memo<u16>,
    month: Memo<u8>,
    days: Memo<Vec<CalendarDayEntry>>,
    using_lff: Memo<bool>,
    show_black_letter: Memo<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarDayEntry {
    month: u8,
    day: u8,
    black_letter_days: Vec<(Feast, String)>,
    listing: Option<(String, LiturgicalDay)>,
    alternatives: Vec<(String, Feast)>,
    other_notes: Vec<(Feast, String)>,
}

// TODO move to server
pub fn calendar_data(cx: Scope, _params: Memo<ParamsMap>, location: Location) -> CalendarData {
    use calendar::{BCP1979_CALENDAR, LFF2018_CALENDAR};

    let tz = get_timezone_offset(cx);
    let yymm = move || {
        location
            .query
            .with(|q| {
                q.get("month").and_then(|month| {
                    month.split_once('-').and_then(|(yyyy, mm)| {
                        match (yyyy.parse::<u16>().ok(), mm.parse::<u8>().ok()) {
                            (Some(y), Some(m)) => Some((y, m)),
                            _ => None,
                        }
                    })
                })
            })
            .unwrap_or_else(|| {
                let today = today(&tz);
                (today.year(), today.month())
            })
    };
    let year = create_memo(cx, move |_| yymm().0);
    let month = create_memo(cx, move |_| yymm().1);

    // defaults to `true` unless ?calendar=bcp
    let using_lff = create_memo(cx, move |_| {
        location
            .query
            .with(|q| q.get("calendar").map(|cal| cal != "bcp"))
            .unwrap_or(true)
    });

    // defaults to `true` unless ?blackletter=false
    let show_black_letter = create_memo(cx, move |_| {
        location
            .query
            .with(|q| q.get("blackletter").map(|bl| bl != "false"))
            .unwrap_or(true)
    });

    let days = create_memo(cx, move |_| {
        let locale = use_locale(cx).get();

        let day_1 = Date::from_ymd(year(), month(), 1);

        let calendar = if using_lff() {
            LFF2018_CALENDAR
        } else {
            BCP1979_CALENDAR
        };

        let language = Language::from_locale(&locale);

        (0..=31)
            .filter_map(|offset| {
                let current_date = day_1.add_days(offset);
                if current_date.year() == year() && current_date.month() == month() {
                    let liturgical_day = calendar.liturgical_day(current_date, false);
                    let rank = calendar.rank(&liturgical_day);

                    let other_notes = liturgical_day
                        .holy_days
                        .iter()
                        .filter(|feast| calendar.feast_day_rank(feast) == Rank::EmberDay)
                        .map(|feast| {
                            (
                                *feast,
                                summary::localize_day_name(
                                    &liturgical_day,
                                    &LiturgicalDayId::Feast(*feast),
                                    &calendar,
                                    language,
                                ),
                            )
                        })
                        .collect();

                    let alternatives = liturgical_day
                        .alternative_services
                        .iter()
                        .map(|feast| {
                            (
                                summary::localize_day_name(
                                    &liturgical_day,
                                    &LiturgicalDayId::Feast(*feast),
                                    &calendar,
                                    language,
                                ),
                                *feast,
                            )
                        })
                        .collect();

                    let black_letter_days = liturgical_day
                        .holy_days
                        .iter()
                        .filter(|feast| {
                            show_black_letter()
                                && calendar.feast_day_rank(feast) == Rank::OptionalObservance
                        })
                        .map(|feast| {
                            (
                                *feast,
                                calendar
                                    .feast_name(*feast, language)
                                    .unwrap_or_else(|| feast.to_string()),
                            )
                        })
                        .collect();

                    let marked_on_calendar = if rank >= Rank::HolyDay {
                        let localized_day_name = summary::localize_day_name(
                            &liturgical_day,
                            &liturgical_day.observed,
                            &calendar,
                            language,
                        );
                        Some((localized_day_name, liturgical_day))
                    } else {
                        None
                    };

                    Some(CalendarDayEntry {
                        month: current_date.month(),
                        day: current_date.day(),
                        black_letter_days,
                        listing: marked_on_calendar,
                        alternatives,
                        other_notes,
                    })
                } else {
                    None
                }
            })
            .collect()
    });

    CalendarData {
        year,
        month,
        days,
        using_lff,
        show_black_letter,
    }
}

#[component]
pub fn Calendar(cx: Scope) -> Element {
    let (t, _) = use_i18n(cx);
    set_title(cx, t("calendar"));
    let data = use_loader::<CalendarData>(cx);
    let (settings_open, set_settings_open) = create_signal(cx, false);

    view! {
        <div>
            <header>
                <span></span>
                <h1>{t("menu.calendar")}</h1>
                <button on:click=move |_| set_settings_open(|n| *n = !*n)>
                    <img src={Icon::Settings.to_string()} alt={t("settings.title")}/>
                </button>
                <Modal
                    open=settings_open
                    on_close=move || set_settings_open(|n| *n = false)
                >
                    <Form>
                        <label class="stacked">
                            {t("calendar.month")}
                            <input type="month" name="month" value={move || format!("{:04}-{:02}", (data.year)(), (data.month)())}/>
                        </label>

                        // Black Letter Days
                        <fieldset class="horizontal">
                            <legend>{t("menu.calendar")}</legend>
                            <label class="horizontal">
                                {t("bcp_1979")}
                                <input type="radio" name="calendar" value="bcp" checked={move || !(data.using_lff)()} />
                            </label>
                            <label class="horizontal">
                                {t("lff_2018")}
                                <input type="radio" name="calendar" value="lff" checked={move || (data.using_lff)()} />
                            </label>
                        </fieldset>

                        // Black Letter Days
                        <label class="horizontal">
                            {t("calendar.omit_black_letter")}
                            <input type="checkbox" name="blackletter" value="false" checked={move || !(data.show_black_letter)()} />
                        </label>

                        <input type="submit" slot="close-button" data-modal-close="settings" value={t("settings.submit")}/>
                    </Form>
                </Modal>
            </header>
            <main>
                <div class="controls">
                    <AdjacentMonth increase=false/>
                    <h2>{move || t(&format!("lectionary.month_{}", (data.month)()))}</h2>
                    <AdjacentMonth increase=true/>
                </div>
                <time class="month" datetime={move || format!("{}-{:02}", (data.year)(), (data.month)())}>
                    <div class="weekday-labels">
                        <div class="weekday-label">{t("canticle_table.sunday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.monday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.tuesday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.wednesday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.thursday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.friday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.saturday_abbrev")}</div>
                    </div>
                    <Weeks/>
                </time>
            </main>
        </div>
    }
}

#[component]
fn AdjacentMonth(cx: Scope, increase: bool) -> Element {
    let (t, _) = use_i18n(cx);
    let curr = use_loader::<CalendarData>(cx);

    let year = move || {
        if (curr.month)() == 1 && !increase {
            (curr.year)() - 1
        } else if (curr.month)() == 12 && increase {
            (curr.year)() + 1
        } else {
            (curr.year)()
        }
    };

    let month = move || {
        if (curr.month)() == 1 && !increase {
            12
        } else if (curr.month)() == 12 && increase {
            1
        } else if !increase {
            (curr.month)() - 1
        } else {
            (curr.month)() + 1
        }
    };

    let calendar = move || {
        if (curr.using_lff)() {
            Some("lff2018".to_string())
        } else {
            None
        }
    };

    let blackletter = move || {
        if (curr.show_black_letter)() {
            None
        } else {
            Some("false".to_string())
        }
    };

    let label = move || {
        if increase && (curr.month)() == 12 {
            format!("{} {}", t("lectionary.month_1"), (curr.year)() + 1)
        } else if increase {
            t(&format!("lectionary.month_{}", (curr.month)() + 1))
        } else if (curr.month)() == 1 {
            format!("{} {}", t("lectionary.month_12"), (curr.year)() - 1)
        } else {
            t(&format!("lectionary.month_{}", (curr.month)() - 1))
        }
    };

    view! {
        <Form method="GET".into()>
            <input type="hidden" name="month" value={move || format!("{:04}-{:02}", year(), month())}/>
            <input type="hidden" name="calendar" value={calendar}/>
            <input type="hidden" name="blackletter" value={blackletter}/>
            <input type="submit" value={label}/>
        </Form>
    }
}

#[component]
fn Weeks(cx: Scope) -> impl IntoChild {
    let data = use_loader::<CalendarData>(cx);
    move || {
        let year = (data.year)();
        data.days.with(|days| {
            let padding_days = Date::from_ymd(year, (data.month)(), 1)
                .weekday()
                .num_days_from_sunday();

            let padding = (1..=padding_days).map(|_| None);
            let days = days.iter().cloned().map(Some);
            padding
                .chain(days)
                .chunks(7)
                .into_iter()
                .map(|chunk| {
                    let week = chunk.collect::<Vec<_>>();
                    view! {
                        <Week year week />
                    }
                })
                .collect::<Vec<_>>()
        })
    }
}

#[component]
fn Week(cx: Scope, year: u16, week: Vec<Option<CalendarDayEntry>>) -> Element {
    let days = week
        .into_iter()
        .map(|day| match day {
            None => view! { <div class="padding"></div> },
            Some(day) => view! { <Day year day /> },
        })
        .collect::<Vec<_>>();

    view! { <div class="week">{days}</div>}
}

#[component]
fn Day(cx: Scope, year: u16, day: CalendarDayEntry) -> Element {
    let CalendarDayEntry {
        day,
        listing,
        alternatives,
        other_notes,
        black_letter_days,
        month,
        ..
    } = day;

    let date = Date::from_ymd(year, month, day);
    let class = if date.weekday() == Weekday::Sun {
        "day sunday"
    } else {
        "day"
    };

    view! {
        <time
            class={class}
            datetime=format!("{}-{:02}-{:02}", year, month, day)
        >
            <a id=format!("{}/{}", month, day)></a>
            <div class="month-number">{day}</div>
            <Listing
                year
                month
                day
                listing
                alternatives
            />
            <BlackLetterDays
                year
                month
                day
                black_letter_days
            />
            <OtherNotes
                year
                month
                day
                other_notes
            />
        </time>
    }
}

#[component]
fn Listing(
    cx: Scope,
    year: u16,
    month: u8,
    day: u8,
    listing: Option<(String, LiturgicalDay)>,
    alternatives: Vec<(String, Feast)>,
) -> Option<Element> {
    let locale = use_locale(cx);
    let (t, _) = use_i18n(cx);

    if let Some((day_name, liturgical_day)) = listing {
        let transferred = if matches!(
            liturgical_day.observed,
            LiturgicalDayId::TransferredFeast(_)
        ) {
            Some(
                view! {<span class="transferred">{format!(" {}", t("daily_readings.transferred"))}</span>},
            )
        } else {
            None
        };

        let alternatives = if alternatives.is_empty() {
            vec![]
        } else {
            alternatives
                .into_iter()
                .map(|(name, feast)| view! {
                    <a
                        class="alternative"
                        href={move || format!("/{}/readings/eucharist/?date={}-{}-{}&alternate={}", locale.get(), year, month, day, feast)}
                    >
                        {name}
                    </a>
                })
                .collect()
        };

        Some(view! {
            <div class="main-listing">
                <a class="day-name" href={move || format!("/{}/readings/eucharist/?date={}-{}-{}", locale.get(), year, month, day)}>{day_name}</a>
                {transferred}
                {alternatives}
            </div>
        })
    } else {
        None
    }
}

#[component]
fn BlackLetterDays(
    cx: Scope,
    year: u16,
    month: u8,
    day: u8,
    black_letter_days: Vec<(Feast, String)>,
) -> Option<Element> {
    let locale = use_locale(cx);
    if black_letter_days.is_empty() {
        None
    } else {
        let days = black_letter_days
            .into_iter()
            .map(|(feast, name)| {
                let href = move || {
                    format!(
                        "/{}/readings/holy-day/?date={}-{}-{}&id={}",
                        locale.get(),
                        year,
                        month,
                        day,
                        feast
                    )
                };
                view! {
                    <li><a href={href}>{name}</a></li>
                }
            })
            .collect::<Vec<_>>();
        Some(view! {
            <ul class="black-letter-days">{days}</ul>
        })
    }
}

#[component]
fn OtherNotes(
    cx: Scope,
    year: u16,
    month: u8,
    day: u8,
    other_notes: Vec<(Feast, String)>,
) -> Option<Element> {
    let locale = use_locale(cx);
    if other_notes.is_empty() {
        None
    } else {
        let others = other_notes
            .into_iter()
            .map(|(id, name)| {
                let href = move || {
                    format!(
                        "/{}/readings/holy-day/?date={}-{}-{}&id={}",
                        locale.get(),
                        year,
                        month,
                        day,
                        id
                    )
                };
                view! { <li><a href={href}>{name}</a></li> }
            })
            .collect::<Vec<_>>();
        Some(view! {
            <ul class="other-notes">{others}</ul>
        })
    }
}
