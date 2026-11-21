use std::{collections::HashMap};

/// Extracts parameter values from requested `target`.
/// 
/// Once the parameter values are extracted, a `HashMap` is
/// returned containing the values.
/// 
/// The `key` of values are the parameter placeholder without
/// the colon(`:`)
/// 
/// If there is not any parameter, an empty `HashMap` 
/// is returned.
pub(crate) fn path_parameters(
    route_pattern: &str,
    target: &str,
) -> HashMap<String, String> {
    let pattern_segments = route_pattern
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    let segments = target
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    let mut parameters = HashMap::new();

    for (pattern, value) in pattern_segments.iter().zip(segments.iter()) {
        if let Some(key) = pattern.strip_prefix(':') {
            parameters.insert(key.to_string(), value.to_string());
        }
    }

    parameters
}

#[cfg(test)]
mod matching {
    use super::*;

    #[test]
    fn extracted_values_match() {
        let parameters = path_parameters(
            "/test/:id/:name", 
            "/test/5/jhon"
        );

        assert_eq!(parameters.get("id").unwrap(), "5");
        assert_eq!(parameters.get("name").unwrap(), "jhon")
    }

    #[test]
    fn no_parameter_placeholder_returns_none() {
        let parameters = path_parameters("/test", "/test");
        assert!(parameters.is_empty())
    }
}