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

/// Structure to track parsed placeholder options
struct PlaceholderOptions {
    kind: String,
    begin_value: Option<String>,
    end_value: Option<String>,
    min_value: Option<String>,
    max_value: Option<String>,
    leet_speak: bool,
    case_mode: String,
}

impl PlaceholderOptions {
    fn new() -> Self {
        PlaceholderOptions {
            kind: String::new(),
            begin_value: None,
            end_value: None,
            min_value: None,
            max_value: None,
            leet_speak: false,
            case_mode: "mixed".to_string(),
        }
    }
}

/// Parse a single placeholder like {number,min=001,max=333}
pub fn parse_placeholder(placeholder: &str) -> TemplatePart {
    let mut opts = PlaceholderOptions::new();

    // Remove braces
    let content = placeholder.trim_start_matches('{').trim_end_matches('}');
    
    // If there are no key=value pairs, the entire content is the kind
    if !content.contains('=') {
        let trimmed = content.trim();
        match trimmed {
            "word" | "month" | "number" | "shortened" | "extended" => opts.kind = trimmed.to_string(),
            _ => {}
        }
        return TemplatePart {
            kind: opts.kind,
            begin_value: None,
            end_value: None,
            min_value: None,
            max_value: None,
            leet_speak: false,
            case_mode: "mixed".to_string(),
        };
    }

    // Parse each part (can be key=value or just a flag)
    for part in content.split(',') {
        let trimmed_part = part.trim();
        
        // Check if this is a key=value pair
        if trimmed_part.contains('=') {
            let key_val: Vec<&str> = trimmed_part.splitn(2, '=').collect();
            if key_val.len() == 2 {
                let key = key_val[0].trim();
                let value = key_val[1].trim();

                match key {
                    "word" | "month" | "number" | "shortened" | "extended" => {
                        // Only set kind if not already set (first occurrence wins)
                        if opts.kind.is_empty() {
                            opts.kind = key.to_string();
                        }
                    }
                    "begin" | "beginValue" => opts.begin_value = Some(value.to_string()),
                    "end" | "endValue" => opts.end_value = Some(value.to_string()),
                    "min" | "minValue" => opts.min_value = Some(value.to_string()),
                    "max" | "maxValue" => opts.max_value = Some(value.to_string()),
                    "leetSpeak" => opts.leet_speak = value.to_lowercase() == "true",
                    "case" => opts.case_mode = value.to_lowercase(),
                    _ => {}
                }
            }
        } else {
            // This is a flag without a value - check if it's one of our known flags
            match trimmed_part {
                // Only set as kind if not already set (first occurrence wins)
                "shortened" | "extended" | "word" | "month" | "number" => {
                    if opts.kind.is_empty() {
                        opts.kind = trimmed_part.to_string();
                    }
                }
                _ => {}
            }
        }
    }

    TemplatePart {
        kind: opts.kind,
        begin_value: opts.begin_value,
        end_value: opts.end_value,
        min_value: opts.min_value,
        max_value: opts.max_value,
        leet_speak: opts.leet_speak,
        case_mode: opts.case_mode,
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
