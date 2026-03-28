/// Template part with all customizable properties
pub struct TemplatePart {
    pub kind: String,
    pub min_size: usize,
    pub max_size: usize,
    pub begin: Option<String>,
    pub end: Option<String>,
    pub leet_speak: bool,
    pub case_mode: String,
}

/// Parse template like {month,maxSize=5,minSize=3,begin=january,end=april,leetSpeak=false,case=all}
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

/// Parse a single placeholder like {month,maxSize=5,minSize=3,begin=january,end=april}
pub fn parse_placeholder(placeholder: &str) -> TemplatePart {
    let mut kind = String::new();
    let mut min_size = 1;
    let mut max_size = 5;
    let mut begin = None;
    let mut end = None;
    let mut leet_speak = false;
    let mut case_mode = "mixed".to_string();
    
    // Remove braces
    let content = placeholder.trim_start_matches('{').trim_end_matches('}');
    
    // If there are no key=value pairs, the entire content is the kind (e.g., "month", "number", or "word")
    if !content.contains('=') {
        let trimmed = content.trim();
        match trimmed {
            "word" | "month" | "number" => kind = trimmed.to_string(),
            _ => {}
        }
        return TemplatePart {
            kind,
            min_size,
            max_size,
            begin,
            end,
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
                "word" | "month" => kind = key.to_string(),
                "minSize" => min_size = value.parse().unwrap_or(1),
                "maxSize" => max_size = value.parse().unwrap_or(5),
                "begin" => begin = Some(value.to_lowercase()),
                "end" => end = Some(value.to_lowercase()),
                "leetSpeak" => leet_speak = value.to_lowercase() == "true",
                "case" => case_mode = value.to_lowercase(),
                _ => {}
            }
        } else if part.trim().contains("word") || part.trim().contains("month") {
            kind = part.trim().to_string();
        }
    }
    
    TemplatePart {
        kind,
        min_size,
        max_size,
        begin,
        end,
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
    fn test_parse_template_with_parameters() {
        let template = "{month,maxSize=5,minSize=3,begin=january,end=april}";
        let parts = parse_template(template);
        
        assert_eq!(parts.len(), 1);
        let part = &parts[0];
        assert_eq!(part.kind, "month");
        assert_eq!(part.max_size, 5);
        assert_eq!(part.min_size, 3);
        assert_eq!(part.begin, Some("january".to_string()));
        assert_eq!(part.end, Some("april".to_string()));
    }

    #[test]
    fn test_parse_template_multiple_parts() {
        let template = "{month}Example{number}";
        let parts = parse_template(template);
        
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].kind, "month");
        assert_eq!(parts[1].kind, "number");
    }

    #[test]
    fn test_parse_placeholder_with_all_options() {
        let placeholder = "{word,maxSize=4,minSize=2,leetSpeak=true,case=all}";
        let part = parse_placeholder(placeholder);
        
        assert_eq!(part.kind, "word");
        assert_eq!(part.max_size, 4);
        assert_eq!(part.min_size, 2);
        assert!(part.leet_speak);
        assert_eq!(part.case_mode, "all");
    }

    #[test]
    fn test_parse_placeholder_with_month_keyword() {
        let placeholder = "{month,maxSize=3}";
        let part = parse_placeholder(placeholder);
        
        assert_eq!(part.kind, "month");
        assert_eq!(part.max_size, 3);
    }
}