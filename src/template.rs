/// Template part with all customizable properties
pub struct TemplatePart {
    pub kind: String,
    pub begin_value: Option<String>,
    pub end_value: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub leet_speak: bool,
    pub case_mode: String,
}

/// Parse template like "prefix{number,min=001,max=333}suffix"
pub fn parse_template(template: &str) -> Vec<TemplatePart> {
    let mut parts = Vec::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '{' {
            // Find the closing brace
            let mut depth = 1;
            let mut j = i + 1;
            while j < chars.len() && depth > 0 {
                if chars[j] == '{' {
                    depth += 1;
                } else if chars[j] == '}' {
                    depth -= 1;
                }
                j += 1;
            }

            if depth == 0 {
                let placeholder = template[i..j].to_string();

                // Parse placeholder content
                let part = parse_placeholder(&placeholder);
                if !part.kind.is_empty() {
                    parts.push(part);
                }
            }

            i = j;
        } else {
            i += 1;
        }
    }

    parts
}

/// Parse a single placeholder like {number,min=001,max=333}
pub fn parse_placeholder(placeholder: &str) -> TemplatePart {
    let mut kind = String::new();
    let mut begin_value = None;
    let mut end_value = None;
    let mut min_value = None;
    let mut max_value = None;
    let mut leet_speak = false;
    let mut case_mode = "mixed".to_string();

    // Remove braces
    let content = placeholder.trim_start_matches('{').trim_end_matches('}');

    // If there are no key=value pairs, the entire content is the kind
    if !content.contains('=') {
        let trimmed = content.trim();
        match trimmed {
            "word" | "month" | "number" | "shortened" | "extended" => kind = trimmed.to_string(),
            _ => {}
        }
        return TemplatePart {
            kind,
            begin_value,
            end_value,
            min_value,
            max_value,
            leet_speak,
            case_mode,
        };
    }

    // Parse each key=value pair
    for part in content.split(',') {
        let key_val: Vec<&str> = part.splitn(2, '=').collect();
        if key_val.len() == 2 {
            let key = key_val[0].trim();
            let value = key_val[1].trim();

            match key {
                "word" | "month" | "number" | "shortened" | "extended" => kind = key.to_string(),
                "begin" | "beginValue" => begin_value = Some(value.to_string()),
                "end" | "endValue" => end_value = Some(value.to_string()),
                "min" | "minValue" => min_value = Some(value.to_string()),
                "max" | "maxValue" => max_value = Some(value.to_string()),
                "leetSpeak" => leet_speak = value.to_lowercase() == "true",
                "case" => case_mode = value.to_lowercase(),
                _ => {}
            }
        } else if part.trim().contains("word")
            || part.trim().contains("month")
            || part.trim().contains("number")
            || part.trim().contains("year")
            || part.trim().contains("shortened")
            || part.trim().contains("extended")
        {
            kind = part.trim().to_string();
        }
    }

    TemplatePart {
        kind,
        begin_value,
        end_value,
        min_value,
        max_value,
        leet_speak,
        case_mode,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_template_simple() {
        let template = "{month}";
        let parts = parse_template(template);

        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].kind, "month");
    }

    #[test]
    fn test_parse_template_with_min_max() {
        let template = "{number,min=001,max=333}";
        let parts = parse_template(template);

        assert_eq!(parts.len(), 1);
        let part = &parts[0];
        assert_eq!(part.kind, "number");
        assert_eq!(part.min_value, Some("001".to_string()));
        assert_eq!(part.max_value, Some("333".to_string()));
    }

    #[test]
    fn test_parse_template_with_prefix_suffix() {
        let template = "prefix{number,min=001,max=333}suffix";
        let parts = parse_template(template);

        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].kind, "number");
    }

    #[test]
    fn test_parse_placeholder_with_all_options() {
        let placeholder = "{word,min=aaa,max=zzz,leetSpeak=true,case=all}";
        let part = parse_placeholder(placeholder);

        assert_eq!(part.kind, "word");
        assert_eq!(part.min_value, Some("aaa".to_string()));
        assert_eq!(part.max_value, Some("zzz".to_string()));
        assert!(part.leet_speak);
        assert_eq!(part.case_mode, "all");
    }

    #[test]
    fn test_parse_placeholder_shortened() {
        let placeholder = "{shortened,min=3}";
        let part = parse_placeholder(placeholder);

        assert_eq!(part.kind, "shortened");
        assert_eq!(part.min_value, Some("3".to_string()));
    }

    #[test]
    fn test_parse_placeholder_extended() {
        let placeholder = "{extended,max=10}";
        let part = parse_placeholder(placeholder);

        assert_eq!(part.kind, "extended");
        assert_eq!(part.max_value, Some("10".to_string()));
    }
}
