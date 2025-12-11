pub fn extract_m_macro(s: &str) -> Option<(usize, usize, String)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i + 2 < bytes.len() {
        if bytes[i] == b'M' && bytes[i + 1] == b'!' && bytes[i + 2] == b'(' {
            let start_content = i + 3;
            let mut depth = 1;
            let mut j = start_content;

            while j < bytes.len() {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end_content = j;
                            let content = s[start_content..end_content].to_string();
                            return Some((i, j + 1, content));
                        }
                    }
                    _ => {}
                }
                j += 1;
            }

            return None;
        }

        i += 1;
    }

    None
}

pub fn extract_v_macro(s: &str) -> Option<(usize, usize, String)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i + 2 < bytes.len() {
        if bytes[i] == b'V' && bytes[i + 1] == b'!' && bytes[i + 2] == b'(' {
            let start_content = i + 3;
            let mut depth = 1;
            let mut j = start_content;

            while j < bytes.len() {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end_content = j;
                            let content = s[start_content..end_content].to_string();
                            return Some((i, j + 1, content));
                        }
                    }
                    _ => {}
                }
                j += 1;
            }

            return None;
        }

        i += 1;
    }

    None
}

pub fn extract_b_macro(s: &str) -> Option<(usize, usize, String)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i + 2 < bytes.len() {
        if bytes[i] == b'B' && bytes[i + 1] == b'!' && bytes[i + 2] == b'(' {
            let start_content = i + 3;
            let mut depth = 1;
            let mut j = start_content;

            while j < bytes.len() {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end_content = j;
                            let content = s[start_content..end_content].trim().to_string();
                            return Some((i, j + 1, content));
                        }
                    }
                    _ => {}
                }
                j += 1;
            }

            return None;
        }

        i += 1;
    }

    None
}

pub fn extract_t_macro(s: &str) -> Option<(usize, usize, String)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i + 2 < bytes.len() {
        if bytes[i] == b'T' && bytes[i + 1] == b'!' && bytes[i + 2] == b'(' {
            let start_content = i + 3;
            let mut depth = 1;
            let mut j = start_content;

            while j < bytes.len() {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end_content = j;
                            let content = s[start_content..end_content].trim().to_string();
                            return Some((i, j + 1, content));
                        }
                    }
                    _ => {}
                }
                j += 1;
            }

            return None;
        }

        i += 1;
    }

    None
}