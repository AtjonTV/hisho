use std::collections::HashMap;

pub fn render_string(template: String, data: &HashMap<String, String>) -> Option<String> {
    let tp_engine = liquid::ParserBuilder::with_stdlib().build();
    if let Ok(engine) = &tp_engine {
        let tp_template = engine.parse(template.as_str().clone());
        if let Ok(template) = tp_template {
            let new_global = liquid::object!(data.clone());
            let tp_value = template.render(&new_global);
            if let Ok(rendered_value) = tp_value {
                return Some(rendered_value);
            } else {
                println!("Failed to render template: {}", tp_value.err().unwrap());
            }
        } else {
            println!("Failed to parse template: {}", tp_template.err().unwrap());
        }
    } else {
        println!("Failed to create template engine: {}", tp_engine.err().unwrap());
    }
    return None;
}

pub fn render_environment(env: HashMap<String, String>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    for (key, value) in &env {
        render_environment_value(key.clone(), value.clone(), &env, &mut result);
    }
    result
}

pub fn render_environment_value(key: String, value: String, lookup_map: &HashMap<String, String>, result_map: &mut HashMap<String, String>) {
    if let Some(rendered_value) = render_string(value.clone(), lookup_map) {
        result_map.insert(key.clone(), rendered_value);
    } else {
        result_map.insert(key.clone(), value.clone());
    }
}
