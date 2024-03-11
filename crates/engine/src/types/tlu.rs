use std::collections::HashMap;

pub struct Tlu {
    param: HashMap<String, String>,
    vert: String,
    frag: String,
}

impl Tlu {
    pub fn parse(text: String, includes: Option<HashMap<String, String>>) -> Self {
        let mut param = HashMap::new();
        let mut vert = String::new();
        let mut frag = String::new();
        let mut current_section = None;

        for line in text.lines() {
            let line = line.trim();
            if line.trim().starts_with("--") {
                continue;
            }
            if line.starts_with('@') {
                current_section = Some(line);
                continue;
            }

            if let Some(section) = current_section {
                match section {
                    "@tlu" => {
                        if let Some((p, value)) = Self::parse_param(line) {
                            param.insert(p.trim().to_string(), value.trim().to_string());
                        }
                    }
                    "@vert" => {
                        if !line.is_empty() {
                            if Self::include(line, &mut vert, &includes) {
                                continue;
                            }
                            vert.push_str(line);
                            vert.push('\n');
                        }
                    }
                    "@frag" => {
                        if !line.is_empty() {
                            if Self::include(line, &mut frag, &includes) {
                                continue;
                            }
                            frag.push_str(line);
                            frag.push('\n');
                        }
                    }
                    _ => (),
                }
            }
        }

        Tlu {
            param,
            vert: vert.trim_end().to_string(),
            frag: frag.trim_end().to_string(),
        }
    }

    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.param.get(param)
    }

    pub fn params(&self) -> &HashMap<String, String> {
        &self.param
    }

    pub fn vert(&self) -> &str {
        &self.vert
    }

    pub fn frag(&self) -> &str {
        &self.frag
    }

    fn parse_param(line: &str) -> Option<(&str, &str)> {
        let mut parts = line.splitn(2, ' ');
        match (parts.next(), parts.next()) {
            (Some(param), Some(value)) => Some((param.trim(), value.trim())),
            _ => None,
        }
    }

    fn include(
        line: &str,
        section: &mut String,
        includes: &Option<HashMap<String, String>>,
    ) -> bool {
        if line.starts_with("#inc") {
            let file_name = line.split_whitespace().nth(1).unwrap();
            if let Some(includes) = includes {
                if let Some(include) = includes.get(file_name) {
                    for line in include.lines() {
                        section.push_str(line.trim());
                        section.push('\n')
                    }
                }
            }
            return true;
        }
        false
    }
}
