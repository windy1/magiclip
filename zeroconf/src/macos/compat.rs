pub fn normalize_domain(domain: &str) -> String {
    if domain.chars().nth(domain.len() - 1).unwrap() == '.' {
        String::from(&domain[..domain.len() - 1])
    } else {
        String::from(domain)
    }
}
