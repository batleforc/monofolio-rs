use crate::i18n::Language;

pub fn format_display_date(raw: &str, lang: Language) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let date_part = trimmed.split('T').next().unwrap_or(trimmed);
    let mut parts = date_part.split('-');
    let year = parts.next();
    let month = parts.next();
    let day = parts.next();

    let (Some(year), Some(month), Some(day)) = (year, month, day) else {
        return trimmed.to_string();
    };

    let month_index = month.parse::<usize>().ok().filter(|m| (1..=12).contains(m));
    let day_num = day.parse::<u32>().ok();

    let (Some(month_index), Some(day_num)) = (month_index, day_num) else {
        return trimmed.to_string();
    };

    let month_name = match lang {
        Language::Fr => [
            "janvier",
            "fevrier",
            "mars",
            "avril",
            "mai",
            "juin",
            "juillet",
            "aout",
            "septembre",
            "octobre",
            "novembre",
            "decembre",
        ][month_index - 1],
        Language::En => [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ][month_index - 1],
    };

    match lang {
        Language::Fr => format!("{} {} {}", day_num, month_name, year),
        Language::En => format!("{} {}, {}", month_name, day_num, year),
    }
}
