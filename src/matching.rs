//! Provides tools for router macthing.
//! 
//! Several process are necessary for router matching. This
//! module provide necessary "steps" for router matching.

pub(crate) fn is_target(route_pattern: &str, target: &str) -> bool {
    let mut pattern_segments = route_pattern
        .split("/")
        .filter(|segmets| !segmets.is_empty())
        .collect::<Vec<&str>>();

    let mut segments = target        
        .split("/")
        .filter(|segmets| !segmets.is_empty())
        .collect::<Vec<&str>>();

    let has_same_length  = pattern_segments.len() == segments.len();
    let has_same_content;
    let mut idxs = Vec::new();

    for (i, v) in pattern_segments.iter().enumerate() {
        if v.starts_with(":") { idxs.push(i) }
    }
    
    for idx in idxs {
        segments[idx] = "";
        pattern_segments[idx] = "";
    }

    for (i, v) in pattern_segments.iter().enumerate() {
        if segments[i] != *v {
            unimplemented!()
        }
    }

    has_same_content = true;

    has_same_content && has_same_length
}
