//! Provides tools for router macthing.
//! 
//! Several process are necessary for router matching. This
//! module provide necessary "steps" for router matching.

pub(crate) fn is_target(route_pattern: &str, target: &str) -> bool {
    let pattern_segments = route_pattern
        .split("/")
        .filter(|segmets| !segmets.is_empty());

    let target_segments = target        
        .split("/")
        .filter(|segmets| !segmets.is_empty());

    let pattern_segments = pattern_segments.collect::<Vec<_>>();
    let target_segments = target_segments.collect::<Vec<_>>();
    
    if pattern_segments.len() != target_segments.len() {
        return false;
    }
    
    pattern_segments
        .iter()
        .zip(target_segments.iter())
        .all(|(pattern, target)| {
            pattern.starts_with(':') || pattern == target
        })
}
