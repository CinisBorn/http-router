use std::{collections::HashMap};

/// Extracts parameter values from requested `target`.
/// 
/// Once the parameter values are extracted, a `HashMap` is
/// returned containing the values.
/// 
/// The `key` of values are the parameter placeholder without
/// the colon(`:`)
pub(crate) fn path_parameters(
    route_pattern: &str, 
    target: &str
) -> Option<HashMap<String, String>> {
    let pattern_segments = route_pattern
        .split("/")
        .filter(|segmets| !segmets.is_empty())
        .collect::<Vec<&str>>();

    let segments = target        
        .split("/")
        .filter(|segmets| !segmets.is_empty())
        .collect::<Vec<&str>>();

    let keys = pattern_segments.iter()
        .filter(|segment| segment.starts_with(":"))
        .map(|parameters| &parameters[1..])
        .map(|parameters| parameters.to_string())
        .collect::<Vec<String>>();

    if keys.is_empty() {
        return None
    }
    
    let values = pattern_segments
        .iter()
        .enumerate()
        .filter_map(|(idx, chunk)| if chunk.starts_with(":") { Some(idx) } else { None })
        .map(|idx| segments[idx].to_string())
        .collect::<Vec<String>>();

    let mut parameters: HashMap<String, String> = HashMap::new();

    for (idx, key) in keys.into_iter().enumerate() {
        parameters.insert(key, values[idx].to_owned());
    }

    Some(parameters)
}

#[cfg(test)]
mod matching {
    use super::*;

    #[test]
    fn extracted_values_match() {
        let parameters = path_parameters(
            "/test/:id/:name", 
            "/test/5/jhon"
        ).unwrap();

        assert_eq!(parameters.get("id").unwrap(), "5");
        assert_eq!(parameters.get("name").unwrap(), "jhon")
    }

    #[test]
    fn no_parameter_placeholder_returns_none() {
        let parameters = path_parameters("/test", "/test");
        assert!(parameters.is_none())
    }
}