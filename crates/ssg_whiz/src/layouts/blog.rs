use super::layout::Layout;
use crate::{
    Footer, FooterLinks, RenderExtraFooter, Section, extra_footer, page_permalink,
    summaries::{PageSummary, Summary},
};
use dioxus::prelude::*;

fn image_variant_path(path: &str, width: u32, height: u32) -> String {
    if let Some((base, ext)) = path.rsplit_once('.') {
        format!(
            "/processed/{base}-{width}x{height}.{ext}",
            base = base.trim_start_matches('/')
        )
    } else {
        path.to_string()
    }
}

fn supports_resized_variants(path: &str) -> bool {
    path.rsplit_once('.')
        .map(|(_, ext)| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "webp"
            )
        })
        .unwrap_or(false)
}

fn format_display_date(date: &str) -> String {
    let Some((year, rest)) = date.split_once('-') else {
        return date.to_string();
    };
    let Some((month, day)) = rest.split_once('-') else {
        return date.to_string();
    };
    let Ok(year) = year.parse::<i32>() else {
        return date.to_string();
    };
    let Ok(month) = month.parse::<u32>() else {
        return date.to_string();
    };
    let Ok(day) = day.parse::<u32>() else {
        return date.to_string();
    };

    let Some(month_name) = month_name(month) else {
        return date.to_string();
    };
    if day == 0 || day > days_in_month(year, month) {
        return date.to_string();
    }

    format!(
        "{} {}{} {} {}",
        weekday_name(year, month, day),
        day,
        ordinal_suffix(day),
        month_name,
        year
    )
}

fn month_name(month: u32) -> Option<&'static str> {
    match month {
        1 => Some("January"),
        2 => Some("February"),
        3 => Some("March"),
        4 => Some("April"),
        5 => Some("May"),
        6 => Some("June"),
        7 => Some("July"),
        8 => Some("August"),
        9 => Some("September"),
        10 => Some("October"),
        11 => Some("November"),
        12 => Some("December"),
        _ => None,
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn ordinal_suffix(day: u32) -> &'static str {
    match day % 100 {
        11..=13 => "th",
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

fn weekday_name(year: i32, month: u32, day: u32) -> &'static str {
    const OFFSETS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    const NAMES: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    let adjusted_year = if month < 3 { year - 1 } else { year };
    let index = (adjusted_year + adjusted_year / 4 - adjusted_year / 100
        + adjusted_year / 400
        + OFFSETS[(month - 1) as usize]
        + day as i32)
        .rem_euclid(7) as usize;
    NAMES[index]
}

fn page_meta_image(post: &PageSummary) -> Option<String> {
    post.open_graph_image
        .or(post.image)
        .map(|image| image.to_string())
}

#[component]
pub fn BlogPost(post: PageSummary, footer_links: FooterLinks) -> Element {
    let content = crate::markdown::markdown_to_html(post.markdown);
    let display_date = format_display_date(post.date);
    let extra_footer = extra_footer();
    rsx! {
        Layout {
            title: "{post.title}",
            description: "{post.description}",
            url: Some(page_permalink(post.folder)),
            image: page_meta_image(&post),
            section: Section::Blog,
            article {
                class: "mt-24 mb-16 mx-auto max-w-prose px-5 lg:max-w-[81.25ch]",
                div {
                    class: "",
                    h1 {
                        class: "text-4xl font-extrabold leading-tight md:text-5xl",
                        "{post.title}"
                    }
                    div {
                        class: "my-8 flex items-center gap-3",
                        if let Some(author_image) = post.author_image {
                            img {
                                class: "h-11 w-11 shrink-0 rounded-full object-cover",
                                width: "44",
                                height: "44",
                                src: author_image,
                                alt: "Author"
                            }
                        }
                        div {
                            class: "min-w-0 leading-tight",
                            if let Some(author) = post.author {
                                strong {
                                    class: "block text-base font-semibold",
                                    "{author}"
                                }
                            }
                            small {
                                class: "block text-base opacity-70",
                                "{display_date}"
                            }
                        }
                    }
                    div {
                        class: "mb-4 flex items-center justify-between border-y border-base-300 py-2",
                        small {
                            class: "block text-sm font-medium uppercase tracking-wide opacity-70",
                            "Share"
                        }
                        div {
                            class: "flex flex-row gap-1",
                            a {
                                class: "btn btn-ghost btn-sm btn-square",
                                href: "https://twitter.com/intent/tweet?url={page_permalink(post.folder)}",
                                img {
                                    width: "16",
                                    height: "16",
                                    src: "/social-sharing/x-twitter.svg"
                                }
                            }
                            a {
                                class: "btn btn-ghost btn-sm btn-square",
                                href: "https://www.linkedin.com/sharing/share-offsite/?url={page_permalink(post.folder)}",
                                img {
                                    width: "16",
                                    height: "16",
                                    src: "/social-sharing/linkedin.svg"
                                }
                            }
                        }
                    }
                    if let Some(image) = post.image {
                        img {
                            class: "mb-8 block aspect-video w-full rounded-box object-cover shadow-md",
                            src: image,
                            alt: "{post.title}"
                        }
                    }
                    div {
                        class: "prose prose-slate max-w-none prose-pre:overflow-x-auto prose-pre:rounded-xl prose-pre:bg-slate-100 prose-code:font-mono prose-img:max-w-full",
                        dangerous_inner_html: "{content}"
                    }
                }
            }
            if let Some(slot) = extra_footer {
                RenderExtraFooter {
                    slot
                }
            }
            Footer {
                margin_top: "mt-0",
                links: footer_links.clone()
            }
        }
    }
}

#[component]
pub fn BlogList(summary: Summary, footer_links: FooterLinks) -> Element {
    let hero_title = "Latest Insights".to_string();
    let hero_subtitle =
        "Ideas, implementation notes, and product updates from the team.".to_string();

    rsx! {
        Layout {
            title: "Blog",
            description: "Blog",
            mobile_menu: None,
            section: Section::Blog,
            section {
                class: "mx-auto mt-32 mb-12 max-w-5xl px-5 text-center",
                h1 {
                    class: "text-4xl font-extrabold md:text-5xl",
                    "{hero_title}"
                }
                h2 {
                    class: "mt-4 text-lg font-medium opacity-70 md:text-xl",
                    "{hero_subtitle}"
                }
            }
            section {
                class: "mx-auto max-w-4xl px-5 pb-8",
                div {
                    class: "grid gap-4 md:grid-cols-2 md:gap-5",
                    for category in summary.categories {
                        for page in category.pages {
                            article {
                                class: "card mx-auto w-full max-w-sm bg-base-100 ring-1 ring-base-300",
                                figure {
                                    a {
                                        href: "/{page.folder}",
                                        if let Some(image) = page.image {
                                            img {
                                                class: "aspect-video w-full object-cover",
                                                src: if supports_resized_variants(image) {
                                                    image_variant_path(image, 384, 216)
                                                } else {
                                                    image.to_string()
                                                },
                                                srcset: if supports_resized_variants(image) {
                                                    format!(
                                                        "{} 1x, {} 2x",
                                                        image_variant_path(image, 384, 216),
                                                        image_variant_path(image, 768, 432)
                                                    )
                                                } else {
                                                    String::new()
                                                },
                                                sizes: "(min-width: 768px) 384px, 100vw",
                                                width: "384",
                                                height: "216",
                                                loading: "lazy",
                                                alt: "{page.title}",
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "card-body gap-3",
                                    h3 {
                                        class: "card-title text-balance text-xl",
                                        a {
                                            href: "/{page.folder}",
                                            "{page.title}"
                                        }
                                    }
                                    p {
                                        class: "text-sm font-semibold uppercase tracking-wide opacity-60",
                                        "{format_display_date(page.date)}"
                                    }
                                    p {
                                        class: "line-clamp-3 opacity-80",
                                        "{page.description}"
                                    }
                                    div {
                                        class: "card-actions justify-end pt-2",
                                        a {
                                            class: "btn btn-outline btn-sm",
                                            href: "/{page.folder}",
                                            "Read More"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Footer {
                links: footer_links
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(image: Option<&'static str>, open_graph_image: Option<&'static str>) -> PageSummary {
        PageSummary {
            date: "2026-03-16",
            title: "Test page",
            description: "Test description",
            folder: "blog/test",
            markdown: "# Test",
            image,
            open_graph_image,
            author: None,
            author_image: None,
        }
    }

    #[test]
    fn formats_iso_dates_with_weekday_month_and_ordinal() {
        assert_eq!(format_display_date("2026-03-09"), "Monday 9th March 2026");
        assert_eq!(format_display_date("2026-05-23"), "Saturday 23rd May 2026");
    }

    #[test]
    fn formats_ordinal_edge_cases() {
        assert_eq!(format_display_date("2026-05-01"), "Friday 1st May 2026");
        assert_eq!(format_display_date("2026-05-02"), "Saturday 2nd May 2026");
        assert_eq!(format_display_date("2026-05-03"), "Sunday 3rd May 2026");
        assert_eq!(format_display_date("2026-05-11"), "Monday 11th May 2026");
        assert_eq!(format_display_date("2026-05-12"), "Tuesday 12th May 2026");
        assert_eq!(format_display_date("2026-05-13"), "Wednesday 13th May 2026");
    }

    #[test]
    fn formats_valid_leap_day() {
        assert_eq!(
            format_display_date("2024-02-29"),
            "Thursday 29th February 2024"
        );
    }

    #[test]
    fn falls_back_to_raw_date_for_unexpected_values() {
        assert_eq!(format_display_date("May 23 2026"), "May 23 2026");
        assert_eq!(format_display_date("2026-02-29"), "2026-02-29");
        assert_eq!(format_display_date("2026-13-01"), "2026-13-01");
    }

    #[test]
    fn page_meta_image_prefers_dedicated_open_graph_image() {
        assert_eq!(
            page_meta_image(&page(
                Some("/blog/test/header.png"),
                Some("/blog/test/open-graph.png")
            )),
            Some("/blog/test/open-graph.png".to_string())
        );
    }

    #[test]
    fn page_meta_image_falls_back_to_visible_image() {
        assert_eq!(
            page_meta_image(&page(Some("/blog/test/header.png"), None)),
            Some("/blog/test/header.png".to_string())
        );
    }
}
