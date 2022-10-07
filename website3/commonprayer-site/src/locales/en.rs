use std::rc::Rc;

use common_macros::hash_map;

use crate::i18n::{I18nArgs, LocaleData};

pub fn en() -> LocaleData {
    LocaleData(
        "en",
        hash_map! {
            //  Common UI Words
            "common_prayer" => Rc::new(|_| "Common Prayer".to_string()) as Rc<dyn Fn(Option<I18nArgs>) -> String>,
            "loading" => Rc::new(|_| "Loading...".to_string()),
            "or" => Rc::new(|_| "or".to_string()),
            "search" => Rc::new(|_| "Search".to_string()),
            "today" => Rc::new(|_| "Today".to_string()),
            "close" => Rc::new(|_| "Close".to_string()),
            "calendar" => Rc::new(|_| "Calendar".to_string()),
            "go" => Rc::new(|_| "Go".to_string()),

            //  404
            "page-404-title" => Rc::new(|_| "Page Not Found".to_string()),
            "page-404-uh_oh" => Rc::new(|_| "Uh-oh!".to_string()),
            "page-404-explanation" => Rc::new(|_| "It seems that the page you’re looking for is missing. If you think this is a mistake, please send me a message using the link below. Otherwise, try again!".to_string()),
            "page-404-email_greg" => Rc::new(|_| "Email Me".to_string()),
            "page-404-button_text" => Rc::new(|_| "Go to the Table of Contents".to_string()),

            //  Menu
            "menu" => Rc::new(|_| "Menu".to_string()),
            "menu-open" => Rc::new(|_| "Open Menu".to_string()),
            "menu-home" => Rc::new(|_| "Home".to_string()),
            "menu-calendar" => Rc::new(|_| "Calendar".to_string()),
            "menu-readings" => Rc::new(|_| "Readings".to_string()),
            "menu-canticle-table" => Rc::new(|_| "Canticle Table".to_string()),
            "menu-hymnal" => Rc::new(|_| "Hymnal".to_string()),
            "menu-psalter" => Rc::new(|_| "Psalter".to_string()),
            "menu-about" => Rc::new(|_| "About".to_string()),

            //  Auth
            "auth-title" => Rc::new(|_| "User Authentication".to_string()),
            "auth-logout" => Rc::new(|_| "Sign Out".to_string()),

            //  Biblical Citation Loader
            "biblical-citation-error" => Rc::new(|args| format!("Something went wrong while loading {}.", args.unwrap().get("citation").cloned().unwrap_or_else(|| "the reading".into()))),
            "biblical-citation-try-again" => Rc::new(|_| "Try Again.".to_string()),

            // // Table of Contents
            "toc-table_of_contents" => Rc::new(|_| "Table of Contents".to_string()),
            "toc-work_in_progress" => Rc::new(|_| "(Work in Progress)".to_string()),
            "toc-calendar_full" => Rc::new(|_| "The Calendar of the Church Year".to_string()),
            "toc-readings" => Rc::new(|_| "Readings".to_string()),
            "toc-daily_readings" => Rc::new(|_| "Daily Readings".to_string()),
            "toc-holy_days" => Rc::new(|_| "Holy Days".to_string()),
            "toc-daily_office" => Rc::new(|_| "Daily Office".to_string()),
            "toc-pray_daily_office" => Rc::new(|_| "Pray the Office".to_string()),
            "toc-daily_office_lectionary" => Rc::new(|_| "Daily Office Lectionary".to_string()),
            "toc-morning_prayer" => Rc::new(|_| "Morning Prayer".to_string()),
            "toc-noonday_prayer" => Rc::new(|_| "Noonday Prayer".to_string()),
            "toc-evening_prayer" => Rc::new(|_| "Evening Prayer".to_string()),
            "toc-compline" => Rc::new(|_| "Compline".to_string()),
            "toc-canticle_table" => Rc::new(|_| "Table of Suggested Canticles".to_string()),
            "toc-great_litany" => Rc::new(|_| "Great Litany".to_string()),
            "toc-proper_liturgies" => Rc::new(|_| "Proper Liturgies for Special Days".to_string()),
            "toc-holy_baptism" => Rc::new(|_| "Holy Baptism".to_string()),
            "toc-holy_eucharist" => Rc::new(|_| "Holy Eucharist".to_string()),
            "toc-pastoral_offices" => Rc::new(|_| "Pastoral Offices".to_string()),
            "toc-marriage" => Rc::new(|_| "Marriage".to_string()),
            "toc-burial" => Rc::new(|_| "Burial".to_string()),
            "toc-episcopal_services" => Rc::new(|_| "Episcopal Services".to_string()),
            "toc-occasional_services" => Rc::new(|_| "Occasional Services".to_string()),
            "toc-psalter_full" => Rc::new(|_| "The Psalter, or Psalms of David".to_string()),
            "toc-prayers_and_thanksgivings" => Rc::new(|_| "Prayers and Thanksgivings".to_string()),
            "toc-catechism_full" => Rc::new(|_| "An Outline of the Faith, or Catechism".to_string()),
            "toc-historical_documents" => Rc::new(|_| "Historical Documents of the Church".to_string()),

            //// Liturgical Terms

            //  Common Terms
            "liturgy" => Rc::new(|_| "Liturgy".to_string()),
            "date" => Rc::new(|_| "Date".to_string()),
            "pray" => Rc::new(|_| "Pray".to_string()),
            "psalter" => Rc::new(|_| "Psalter".to_string()),
            "traditional" => Rc::new(|_| "Traditional".to_string()),
            "contemporary" => Rc::new(|_| "Contemporary".to_string()),
            "rite_i" => Rc::new(|_| "Rite I".to_string()),
            "rite_ii" => Rc::new(|_| "Rite II".to_string()),
            "eow" => Rc::new(|_| "EOW".to_string()),
            "bcp_1979" => Rc::new(|_| "Book of Common Prayer (1979)".to_string()),
            "bcp_1979_abbrev" => Rc::new(|_| "BCP (1979)".to_string()),
            "lff" => Rc::new(|_| "Lesser Feasts & Fasts".to_string()),
            "lff_2018" => Rc::new(|_| "Lesser Feasts & Fasts (2018)".to_string()),
            "lff_2018_abbrev" => Rc::new(|_| "LFF (2018)".to_string()),
            "eow_1" => Rc::new(|_| "Enriching Our Worship".to_string()),
            "collects" => Rc::new(|_| "Collects".to_string()),
            "collects_contemporary" => Rc::new(|_| "Collects (Contemporary)".to_string()),
            "collects_traditional" => Rc::new(|_| "Collects (Traditional)".to_string()),

            //  Bible Versions
            "bible-version-NRSV-full" => Rc::new(|_| "New Revised Standard Version (NRSV)".to_string()),
            "bible-version-ESV-full" => Rc::new(|_| "English Standard Version (ESV)".to_string()),
            "bible-version-CEB-full" => Rc::new(|_| "Common English Bible (CEB)".to_string()),
            "bible-version-KJV-full" => Rc::new(|_| "King James Version (KJV)".to_string()),
            "bible-version-RV-full" => Rc::new(|_| "Reina-Valera (1909)".to_string()),
            "bible-version-NRSV" => Rc::new(|_| "NRSV".to_string()),
            "bible-version-ESV" => Rc::new(|_| "ESV".to_string()),
            "bible-version-CEB" => Rc::new(|_| "CEB".to_string()),
            "bible-version-KJV" => Rc::new(|_| "KJV".to_string()),
            "bible-version-RV" => Rc::new(|_| "RV".to_string()),

            //  Liturgy Versions
            "version-BCP1979" => Rc::new(|_| "Book of Common Prayer (1979)".to_string()),
            "version-EOW" => Rc::new(|_| "Enriching Our Worship".to_string()),
            "version-Expansive" => Rc::new(|_| "Expansive-Language".to_string()),
            "version-RiteI" => Rc::new(|_| "Rite I (Traditional Language)".to_string()),
            "version-RiteII" => Rc::new(|_| "Rite II (Contemporary Language)".to_string()),
            "version-Parallel" => Rc::new(|_| "Parallels".to_string()),

            //  Citations
            "source" => Rc::new(|_| "Source".to_string()),
            "reference" => Rc::new(|_| "{ $source} p. { $page}".to_string()),

            //  Export
            "export-export" => Rc::new(|_| "Export".to_string()),
            "export-link" => Rc::new(|_| "Share Link".to_string()),
            "export-embed" => Rc::new(|_| "Embed Code".to_string()),
            "export-word" => Rc::new(|_| "Open in Word".to_string()),
            "export-venite" => Rc::new(|_| "Copy to Venite.app".to_string()),
            "export-json" => Rc::new(|_| "Download (JSON)".to_string()),
            "export-clipboard_success" => Rc::new(|_| "Copied to Clipboard".to_string()),
            "export-clipboard_error" => Rc::new(|_| "Could not copy to clipboard. Try copying and pasting manually.".to_string()),
            "export-selections" => Rc::new(|_| "Selected Text".to_string()),
            "export-whole_doc" => Rc::new(|_| "Whole Document".to_string()),
            "export-select" => Rc::new(|_| "Select Items to Export".to_string()),


            //// Pages
            //  About Page
            "about-title" => Rc::new(|_| "Common Prayer Online".to_string()),
            "about-subtitle" => Rc::new(|_| "An unofficial source for The Episcopal Church’s authorized liturgies.".to_string()),
            "about-email" => Rc::new(|_| "Email Me".to_string()),

            //  Calendar Page
            "calendar-omit_black_letter" => Rc::new(|_| "Omit Days of Optional Observance (“Black-Letter Days”)".to_string()),
            "calendar-month" => Rc::new(|_| "Month".to_string()),
            "calendar-year" => Rc::new(|_| "Year".to_string()),

            //  Canticle Table
            "canticle-swap-change_canticle" => Rc::new(|_| "Change Canticle".to_string()),
            "canticle-swap-choose" => Rc::new(|_| "Choose a Canticle".to_string()),
            "canticle-swap-connection_error" => Rc::new(|_| "There was an error connecting to our server. Are you connected to the Internet?".to_string()),
            "canticle-swap-error" => Rc::new(|_| "Something went wrong while loading canticle choices. Try again later?".to_string()),
            "canticle-swap-close" => Rc::new(|_| "Cancel".to_string()),
            "canticle-swap-any" => Rc::new(|_| "—".to_string()),
            "canticle-table-title" => Rc::new(|_| "Table of Suggested Canticles".to_string()),
            "canticle-table-canticles_mp" => Rc::new(|_| "Suggested Canticles at Morning Prayer".to_string()),
            "canticle-table-canticles_ep" => Rc::new(|_| "Suggested Canticles at Evening Prayer".to_string()),
            "canticle-table-supplemental_materials" => Rc::new(|_| "Supplemental Liturgical Materials and Rite II".to_string()),
            "canticle-table-after_ot" => Rc::new(|_| "After the Old Testament Reading".to_string()),
            "canticle-table-after_nt" => Rc::new(|_| "After the New Testament Reading".to_string()),
            "canticle-table-sunday_abbrev" => Rc::new(|_| "Sun.".to_string()),
            "canticle-table-monday_abbrev" => Rc::new(|_| "Mon.".to_string()),
            "canticle-table-tuesday_abbrev" => Rc::new(|_| "Tue.".to_string()),
            "canticle-table-wednesday_abbrev" => Rc::new(|_| "Wed.".to_string()),
            "canticle-table-thursday_abbrev" => Rc::new(|_| "Thu.".to_string()),
            "canticle-table-friday_abbrev" => Rc::new(|_| "Fri.".to_string()),
            "canticle-table-saturday_abbrev" => Rc::new(|_| "Sat.".to_string()),
            "canticle-table-on_feasts" => Rc::new(|_| "On Feasts of our Lord and other Major Feasts".to_string()),
            "canticle-table-magnificat_note" => Rc::new(|_| "* If only one Reading is used, the suggested Canticle is the Magnificat.".to_string()),
            "canticle-table-magnificat_note_eow" => Rc::new(|_| "** If only one reading is used, the suggested canticle is The Song of Mary.".to_string()),
            "canticle-table-or" => Rc::new(|_| "or".to_string()),
            "canticle-table-lent" => Rc::new(|_| "Lent".to_string()),
            "canticle-table-advent" => Rc::new(|_| "Advent".to_string()),
            "canticle-table-advent_and_lent" => Rc::new(|_| "Advent and Lent".to_string()),
            "canticle-table-christmas" => Rc::new(|_| "Christmas".to_string()),
            "canticle-table-easter" => Rc::new(|_| "Easter".to_string()),
            "canticle-table-please_note" => Rc::new(|_| "Please Note".to_string()),
            "canticle-table-eow_canticle_table_note" => Rc::new(|_| "The published edition includes a handful of typographical errors in the table of suggested canticles. This page reflects a version of the text which has been corrected and conformed in capitalization and punctuation to the Book of Common Prayer.".to_string()),
            "canticle-table-canticles_appointed_for_christmas" => Rc::new(|_| "* Canticles appointed for Christmas may be used through the First Sunday after the Epiphany.".to_string()),
            "canticle-table-canticle_n" => Rc::new(|_| "Canticle { $number }".to_string()),

            //  Daily Readings
            "daily-readings-title" => Rc::new(|_| "Daily Readings".to_string()),
            "daily-readings-readings_for_date" => Rc::new(|_| "Readings for Date".to_string()),
            "daily-readings-daily_readings_error" => Rc::new(|_| "Uh-oh! Something went wrong while loading readings.".to_string()),
            "daily-readings-default" => Rc::new(|_| "(Default)".to_string()),
            "daily-readings-alternate" => Rc::new(|_| "(Alternate)".to_string()),
            "daily-readings-transferred" => Rc::new(|_| "(Transferred)".to_string()),
            "daily-readings-psalms" => Rc::new(|_| "Psalms".to_string()),
            "daily-readings-psalm" => Rc::new(|args| format!("Psalm {}", args.unwrap().get("number").cloned().unwrap_or_default())),
            "daily-readings-canticle" => Rc::new(|_| "Canticle { $number }".to_string()),
            "daily-readings-daily_office_readings" => Rc::new(|_| "Daily Office Readings".to_string()),
            "daily-readings-daily_office_psalms" => Rc::new(|_| "Daily Office Psalms".to_string()),
            "daily-readings-thirty_day_psalms" => Rc::new(|_| "30-Day Psalm Cycle".to_string()),
            "daily-readings-morning" => Rc::new(|_| "Morning".to_string()),
            "daily-readings-evening" => Rc::new(|_| "Evening".to_string()),
            "daily-readings-todays_readings" => Rc::new(|_| "Click here for today’s readings.".to_string()),
            "daily-readings-or" => Rc::new(|_| "or".to_string()),
            "daily-readings-vigil_readings" => Rc::new(|_| "At the Liturgy of the Word".to_string()),
            "daily-readings-eucharist_readings" => Rc::new(|_| "At the Eucharist".to_string()),
            "daily-readings-option" => Rc::new(|_| "Option { $n}".to_string()),

            //  Home
            "home-sunday" => Rc::new(|_| "Sunday".to_string()),
            "home-read-more" => Rc::new(|_| "Read more.".to_string()),
            "home-favorites" => Rc::new(|_| "Favorites".to_string()),
            "document-action-mark_favorite" => Rc::new(|_| "Mark as Favorite".to_string()),
            "document-action-remove_favorite" => Rc::new(|_| "Remove Favorite".to_string()),

            //  Hymnal
            "hymnal-any" => Rc::new(|_| "—".to_string()),
            "hymnal-h82" => Rc::new(|_| "The Hymnal 1982".to_string()),
            "hymnal-levas" => Rc::new(|_| "Lift Every Voice and Sing".to_string()),
            "hymnal-wlp" => Rc::new(|_| "Wonder, Love, and Praise".to_string()),
            "hymnal-el_himnario" => Rc::new(|_| "El Himnario".to_string()),
            "hymnal-h82_abbrev" => Rc::new(|_| "1982".to_string()),
            "hymnal-levas_abbrev" => Rc::new(|_| "LEVAS".to_string()),
            "hymnal-wlp_abbrev" => Rc::new(|_| "WLP".to_string()),
            "hymnal-more_info" => Rc::new(|_| "More information available on".to_string()),
            "hymnal-alt_text" => Rc::new(|_| "Page Scan from Hymnary.org.".to_string()),
            "hymnal-copyright_restriction" => Rc::new(|_| "[This page is unavailable due to copyright.]".to_string()),
            "hymnal-copyright_footer" => Rc::new(|_| "All data and images for this page are taken from publicly-available data from Hymnary.org. To the best of my knowledge, no copyrighted material is reproduced on this page.".to_string()),
            "hymnal-tune" => Rc::new(|_| "Tune".to_string()),
            "hymnal-first_line" => Rc::new(|_| "First Line".to_string()),
            "hymnal-text_title" => Rc::new(|_| "Text Title".to_string()),
            "hymnal-refrain_first_line" => Rc::new(|_| "First Line of Refrain".to_string()),
            "hymnal-authors" => Rc::new(|_| "Author(s)".to_string()),
            "hymnal-composers" => Rc::new(|_| "Composer(s)".to_string()),
            "hymnal-meter" => Rc::new(|_| "Meter".to_string()),
            "hymnal-text_sources" => Rc::new(|_| "Text Source(s)".to_string()),
            "hymnal-tune_sources" => Rc::new(|_| "Tune Source(s)".to_string()),
            "hymnal-music_available" => Rc::new(|_| "Music Available".to_string()),
            "hymnal-page_back" => Rc::new(|_| "Turn Back One Page".to_string()),
            "hymnal-page_forward" => Rc::new(|_| "Turn Forward One Page".to_string()),
            "hymnal-page_n" => Rc::new(|_| "Page { $number}".to_string()),
            "hymnal-text" => Rc::new(|_| "Words".to_string()),
            "hymnal-music" => Rc::new(|_| "Music".to_string()),
            "hymnal-text_view" => Rc::new(|_| "Text".to_string()),
            "hymnal-music_view" => Rc::new(|_| "Music".to_string()),
            "hymnal-video_view" => Rc::new(|_| "Videos".to_string()),
            "hymnal-video_error" => Rc::new(|_| "Oops! Something went wrong when we tried to load videos for this hymn.".to_string()),
            "hymnal-more_results" => Rc::new(|_| "Click here for more search results.".to_string()),
            "hymnal-category_lookup" => Rc::new(|_| "{ $category } Hymns".to_string()),
            "hymnal-search_by_bing" => Rc::new(|_| "Search results provided by the Bing Video Search API.".to_string()),
            "hymnal-search_instruction" => Rc::new(|_| "Begin by searching for a hymn.".to_string()),

            //  Lectionary
            "lectionary-the_lectionary" => Rc::new(|_| "The Lectionary".to_string()),
            "lectionary-month" => Rc::new(|args| match args.unwrap().get("month").unwrap().as_str() {
                "1" => "January",
                "2" => "February",
                "3" => "March",
                "4" => "April",
                "5" => "May",
                "6" => "June",
                "7" => "July",
                "8" => "August",
                "9" => "September",
                "10" => "October",
                "11" => "November",
                "12" => "December",
                _ => panic!()
            }.to_string()),
            "lectionary-collect_not_found" => Rc::new(|_| "Collect not found for this day.".to_string()),
            "lectionary-reading_not_found" => Rc::new(|_| "Reading not found.".to_string()),
            "lectionary-track_one" => Rc::new(|_| "Track One".to_string()),
            "lectionary-track_two" => Rc::new(|_| "Track Two".to_string()),
            "lectionary-lessons_and_psalm" => Rc::new(|_| "Lessons and Psalm".to_string()),
            "lectionary-palms" => Rc::new(|_| "Liturgy of the Palms".to_string()),
            "lectionary-vigil" => Rc::new(|_| "Vigil Readings".to_string()),
            "lectionary-first_lesson" => Rc::new(|_| "First Lesson".to_string()),
            "lectionary-psalm" => Rc::new(|_| "Psalm".to_string()),
            "lectionary-epistle" => Rc::new(|_| "Epistle".to_string()),
            "lectionary-gospel" => Rc::new(|_| "Gospel".to_string()),
            "lectionary-no_readings" => Rc::new(|_| "There were no readings found for this date in the Revised Common Lectionary. Would you like to look at the ".to_string()),
            "lectionary-no_readings_lff" => Rc::new(|_| "There was no holy day found on this date. Would you like to look at the ".to_string()),
            "lectionary-no_readings_end" => Rc::new(|_| "?".to_string()),
            "lectionary-go" => Rc::new(|_| "Go".to_string()),

            //  Meditation
            "meditation-title" => Rc::new(|_| "Meditation".to_string()),
            "meditation-minutes" => Rc::new(|_| "Minutes".to_string()),
            "meditation-seconds" => Rc::new(|_| "Seconds".to_string()),
            "meditation-use_bell" => Rc::new(|_| "Play bell sound to begin and end meditation.".to_string()),
            "meditation-begin" => Rc::new(|_| "Begin".to_string()),
            "meditation-pause" => Rc::new(|_| "Pause".to_string()),
            "meditation-resume" => Rc::new(|_| "Resume".to_string()),
            "meditation-stop" => Rc::new(|_| "Finish".to_string()),
            "meditation-bell_error" => Rc::new(|_| "Error while trying to play audio.".to_string()),
            "meditation-interval_error" => Rc::new(|_| "Error while trying to set timer.".to_string()),
            "meditation-no_script" => Rc::new(|_| "The Meditation Timer cannot run if JavaScript is disabled.".to_string()),

            //  Psalter
            "psalm-psalm" => Rc::new(|_| "Psalm".to_string()),
            "psalm-prev" => Rc::new(|_| "Previous".to_string()),
            "psalm-next" => Rc::new(|_| "Next".to_string()),
            "psalm-en" => Rc::new(|_| "English (BCP 1979)".to_string()),
            "psalm-es" => Rc::new(|_| "Spanish (LOC 1979)".to_string()),
            "psalm-error" => Rc::new(|_| "Something went wrong while trying to load the psalm. Please try to choose a psalm again using the menu.".to_string()),
            "psalter-full_title" => Rc::new(|_| "The Psalter, or Psalms of David".to_string()),
            "psalter-book_one" => Rc::new(|_| "Book One".to_string()),
            "psalter-book_two" => Rc::new(|_| "Book Two".to_string()),
            "psalter-book_three" => Rc::new(|_| "Book Three".to_string()),
            "psalter-book_four" => Rc::new(|_| "Book Four".to_string()),
            "psalter-book_five" => Rc::new(|_| "Book Five".to_string()),
            "psalter-morning" => Rc::new(|_| "Morning Prayer".to_string()),
            "psalter-evening" => Rc::new(|_| "Evening Prayer".to_string()),
            "psalter-day" => Rc::new(|_| "%{nth} Day".to_string()),
            "psalter-ord_1" => Rc::new(|_| "First".to_string()),
            "psalter-ord_2" => Rc::new(|_| "Second".to_string()),
            "psalter-ord_3" => Rc::new(|_| "Third".to_string()),
            "psalter-ord_4" => Rc::new(|_| "Fourth".to_string()),
            "psalter-ord_5" => Rc::new(|_| "Fifth".to_string()),
            "psalter-ord_6" => Rc::new(|_| "Sixth".to_string()),
            "psalter-ord_7" => Rc::new(|_| "Seventh".to_string()),
            "psalter-ord_8" => Rc::new(|_| "Eighth".to_string()),
            "psalter-ord_9" => Rc::new(|_| "Ninth".to_string()),
            "psalter-ord_10" => Rc::new(|_| "Tenth".to_string()),
            "psalter-ord_11" => Rc::new(|_| "Eleventh".to_string()),
            "psalter-ord_12" => Rc::new(|_| "Twelfth".to_string()),
            "psalter-ord_13" => Rc::new(|_| "Thirteenth".to_string()),
            "psalter-ord_14" => Rc::new(|_| "Fourteenth".to_string()),
            "psalter-ord_15" => Rc::new(|_| "Fifteenth".to_string()),
            "psalter-ord_16" => Rc::new(|_| "Sixteenth".to_string()),
            "psalter-ord_17" => Rc::new(|_| "Seventeenth".to_string()),
            "psalter-ord_18" => Rc::new(|_| "Eighteenth".to_string()),
            "psalter-ord_19" => Rc::new(|_| "Nineteenth".to_string()),
            "psalter-ord_20" => Rc::new(|_| "Twentieth".to_string()),
            "psalter-ord_21" => Rc::new(|_| "Twenty-First".to_string()),
            "psalter-ord_22" => Rc::new(|_| "Twenty-Second".to_string()),
            "psalter-ord_23" => Rc::new(|_| "Twenty-Third".to_string()),
            "psalter-ord_24" => Rc::new(|_| "Twenty-Fourth".to_string()),
            "psalter-ord_25" => Rc::new(|_| "Twenty-Fifth".to_string()),
            "psalter-ord_26" => Rc::new(|_| "Twenty-Sixth".to_string()),
            "psalter-ord_27" => Rc::new(|_| "Twenty-Seventh".to_string()),
            "psalter-ord_28" => Rc::new(|_| "Twenty-Eighth".to_string()),
            "psalter-ord_29" => Rc::new(|_| "Twenty-Ninth".to_string()),
            "psalter-ord_30" => Rc::new(|_| "Thirtieth".to_string()),

            //  Search
            "search-enter_query" => Rc::new(|_| "Enter a query in the search bar above to search the Common Prayer library.".to_string()),
            "search-no_results" => Rc::new(|_| "No results were found for your query.".to_string()),
            "search-results_singular" => Rc::new(|_| "Found 1 result.".to_string()),
            "search-results_plural" => Rc::new(|args| format!("Found {} results.", args.get("number").unwrap_or(0))),
            "search-hymn" => Rc::new(|_| "Hymn".to_string()),
            "search-holy_day" => Rc::new(|_| "Holy Day".to_string()),
            "search-document" => Rc::new(|_| "Document".to_string()),
            "search-category" => Rc::new(|_| "Category".to_string()),
            "search-psalm" => Rc::new(|_| "Psalm".to_string()),
            "search-fragment" => Rc::new(|_| "(liturgical text)".to_string()),

            //  Settings
            "settings-dark_mode-label" => Rc::new(|_| "Dark Mode".to_string()),
            "settings-dark_mode-auto" => Rc::new(|_| "Auto".to_string()),
            "settings-dark_mode-auto_explanation" => Rc::new(|_| "Site will match your device’s mode.".to_string()),
            "settings-dark_mode-light" => Rc::new(|_| "Light".to_string()),
            "settings-dark_mode-light_explanation" => Rc::new(|_| "Site will always use light mode.".to_string()),
            "settings-dark_mode-dark" => Rc::new(|_| "Dark".to_string()),
            "settings-dark_mode-dark_explanation" => Rc::new(|_| "Site will always use dark mode.".to_string()),
            "settings-submit" => Rc::new(|_| "Save Settings".to_string()),
            "settings-title" => Rc::new(|_| "Settings".to_string()),
            "settings-general" => Rc::new(|_| "General Settings".to_string()),
            "settings-liturgy" => Rc::new(|_| "Liturgy Settings".to_string()),
            "settings-advanced" => Rc::new(|_| "Advanced Settings".to_string()),
            "settings-version" => Rc::new(|_| "Version".to_string()),
            "settings-calendar" => Rc::new(|_| "Calendar".to_string()),
            "settings-psalm_cycle" => Rc::new(|_| "Psalm Cycle".to_string()),
            "settings-bible_version" => Rc::new(|_| "Bible Version".to_string()),
            "settings-no_bible_version" => Rc::new(|_| "The Bible version can be set for each liturgy below, or a global preference can be chosen here.".to_string()),
            "settings-use_black_letter_collects" => Rc::new(|_| "Use Collects for Black-Letter Days".to_string()),
            "settings-gloria_patri" => Rc::new(|_| "Gloria Patri".to_string()),
            "settings-gloria_patri_2" => Rc::new(|_| "Glory to the Father...".to_string()),
            "settings-gloria_patri_1" => Rc::new(|_| "Glory be to the Father...".to_string()),
            "settings-yes" => Rc::new(|_| "Yes".to_string()),
            "settings-no" => Rc::new(|_| "No".to_string()),
            "settings-rite_i" => Rc::new(|_| "Rite I (Traditional Language)".to_string()),
            "settings-rite_ii" => Rc::new(|_| "Rite II (Contemporary Language)".to_string()),
            "settings-success" => Rc::new(|_| "Your settings have been updated.".to_string()),
            "settings-error" => Rc::new(|_| "Something went wrong while updating your preferences. Try again later.".to_string()),
            "settings-not_available" => Rc::new(|_| "We were unable to save your settings in your browser’s local storage. Is it possible your privacy settings have this disabled?".to_string()),
            "settings-display-settings-title" => Rc::new(|_| "Display Settings".to_string()),
            "settings-display-settings-liturgy" => Rc::new(|_| "Liturgy Display Settings".to_string()),
            "settings-display-settings-bible_verses" => Rc::new(|_| "Show Bible Verse Numbers".to_string()),
            "settings-display-settings-psalm_verses" => Rc::new(|_| "Show Psalm Verse Numbers".to_string()),
            "settings-use_preferences" => Rc::new(|_| "View Current Liturgy with Selected Preferences".to_string()),

            // // Categories
            "category-OpeningSentences" => Rc::new(|_| "Opening Sentences".to_string()),
            "category-ClosingSentences" => Rc::new(|_| "Closing Sentences".to_string()),
            "category-OffertorySentences" => Rc::new(|_| "Offertory Sentences".to_string()),
            "category-InvitatoryAntiphons" => Rc::new(|_| "Invitatory Antiphons".to_string()),
            "category-PrayersAndThanksgivings" => Rc::new(|_| "Prayers and Thanksgivings".to_string()),
            "category-AdditionalPrayers" => Rc::new(|_| "Additional Prayers".to_string()),
            "category-ServiceOfLight" => Rc::new(|_| "Service of Light".to_string()),
            "category-office" => Rc::new(|_| "Daily Office".to_string()),
            "category-burial" => Rc::new(|_| "Burial".to_string()),
            "category-canticle" => Rc::new(|_| "Canticles".to_string()),
            "category-collects" => Rc::new(|_| "Collects".to_string()),
            "category-opening-sentences" => Rc::new(|_| "Opening Sentences".to_string()),
            "category-closing-sentences" => Rc::new(|_| "Closing Sentences".to_string()),
            "category-eucharist" => Rc::new(|_| "Eucharist".to_string()),
            "category-offertory-sentences" => Rc::new(|_| "Offertory Sentences".to_string()),
            "category-invitatory-antiphons" => Rc::new(|_| "Invitatory Antiphons".to_string()),
            "category-marriage" => Rc::new(|_| "Marriage".to_string()),
            "category-prayers-and-thanksgivings" => Rc::new(|_| "Prayers and Thanksgivings".to_string()),
            "lookup-canticle_table" => Rc::new(|_| "Table of Suggested Canticles".to_string()),
            "lookup-collect_of_the_day" => Rc::new(|_| "The Collect of the Day".to_string()),
            "lookup-lectionary_reading" => Rc::new(|_| "Lectionary Readings".to_string()),
            "slug-Version" => Rc::new(|_| "Version".to_string()),
            "slug-ConcerningTheService" => Rc::new(|_| "Concerning the Service".to_string()),
            "slug-AdditionalDirections" => Rc::new(|_| "Additional Directions".to_string()),
            "slug-AdditionalPrayers" => Rc::new(|_| "Additional Prayers".to_string()),
            "slug-Order" => Rc::new(|_| "Order".to_string()),
            "slug-Readings" => Rc::new(|_| "Readings".to_string()),
            "slug-Parallels" => Rc::new(|_| "Parallels".to_string()),
            "slug-Office" => Rc::new(|_| "Daily Office".to_string()),
            "slug-MorningPrayer" => Rc::new(|_| "Morning Prayer".to_string()),
            "slug-NoondayPrayer" => Rc::new(|_| "Noonday Prayer".to_string()),
            "slug-ServiceOfLight" => Rc::new(|_| "Service of Light".to_string()),
            "slug-EveningPrayer" => Rc::new(|_| "Evening Prayer".to_string()),
            "slug-Compline" => Rc::new(|_| "Compline".to_string()),
            "slug-Canticles" => Rc::new(|_| "Canticles".to_string()),
            "slug-SuggestedCanticles" => Rc::new(|_| "Suggested Canticles".to_string()),
            "slug-OpeningSentences" => Rc::new(|_| "Opening Sentences".to_string()),
            "slug-InvitatoryAntiphons" => Rc::new(|_| "Invitatory Antiphons".to_string()),
            "slug-ClosingSentences" => Rc::new(|_| "Closing Sentences".to_string()),
            "slug-Canticle" => Rc::new(|_| "Canticle".to_string()),
            "slug-GreatLitany" => Rc::new(|_| "The Great Litany".to_string()),
            "slug-Collects" => Rc::new(|_| "Collects".to_string()),
            "slug-Baptism" => Rc::new(|_| "Baptism".to_string()),
            "slug-Eucharist" => Rc::new(|_| "Eucharist".to_string()),
            "slug-PrayersOfThePeople" => Rc::new(|_| "Prayers of the People".to_string()),
            "slug-OffertorySentences" => Rc::new(|_| "Offertory Sentences".to_string()),
            "slug-GreatThanksgiving" => Rc::new(|_| "Great Thanksgiving".to_string()),
            "slug-ProperPrefaces" => Rc::new(|_| "Proper Prefaces".to_string()),
            "slug-PenitentialSentences" => Rc::new(|_| "Penitential Sentences".to_string()),
            "slug-ConsecratingAdditional" => Rc::new(|_| "Consecrating Additional Bread and Wine".to_string()),
            "slug-PrayerA" => Rc::new(|_| "Prayer A".to_string()),
            "slug-PrayerB" => Rc::new(|_| "Prayer B".to_string()),
            "slug-PrayerC" => Rc::new(|_| "Prayer C".to_string()),
            "slug-PrayerD" => Rc::new(|_| "Prayer D".to_string()),
            "slug-PrayerI" => Rc::new(|_| "Prayer I".to_string()),
            "slug-PrayerII" => Rc::new(|_| "Prayer II".to_string()),
            "slug-Prayer1" => Rc::new(|_| "Prayer 1".to_string()),
            "slug-Prayer2" => Rc::new(|_| "Prayer 2".to_string()),
            "slug-Prayer3" => Rc::new(|_| "Prayer 3".to_string()),
            "slug-PrayersAndThanksgivings" => Rc::new(|_| "Prayers and Thanksgivings".to_string()),
            "slug-PastoralOffices" => Rc::new(|_| "Pastoral Offices".to_string()),
            "slug-Marriage" => Rc::new(|_| "Marriage".to_string()),
            "slug-CelebrationAndBlessing" => Rc::new(|_| "Celebration and Blessing of a Marriage".to_string()),
            "slug-CivilMarriage" => Rc::new(|_| "Blessing of a Civil Marriage".to_string()),
            "slug-WitnessingAndBlessing" => Rc::new(|_| "Witnessing and Blessing of a Marriage".to_string()),
            "slug-WitnessingAndBlessingLifelongCovenant" => Rc::new(|_| "Witnessing and Blessing of a Lifelong Covenant".to_string()),
            "slug-Burial" => Rc::new(|_| "Burial".to_string()),
            "slug-BurialOfAChild" => Rc::new(|_| "Burial of aChild".to_string()),
            "slug-BurialOfANonChristian" => Rc::new(|_| "Burial of a Non-Christian".to_string()),
            "slug-OccasionalServices" => Rc::new(|_| "Occasional Services".to_string()),
            "slug-Guadalupe" => Rc::new(|_| "Our Lady of Guadalupe".to_string()),
            "slug-Renaming" => Rc::new(|_| "Renaming".to_string()),
            "slug-Creeds" => Rc::new(|_| "Creeds".to_string()),
            "slug-ApostlesCreed" => Rc::new(|_| "Apostles’ Creed".to_string()),
            "slug-NiceneCreed" => Rc::new(|_| "Nicene Creed".to_string()),
        },
    )
}
