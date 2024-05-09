use std::fs;
use std::str;

pub fn read_file<S: AsRef<str>>(file_name: S) -> Result<String, String> {
    match fs::read_to_string(file_name.as_ref()) {
        Ok(content) => Ok(content),
        Err(_) => Err(format!("Error reading file {}", file_name.as_ref())),
    }
}

pub fn preprocess(raw_code: String) -> Result<(String, Vec<(String, String)>), String> {
    let parts = raw_code.as_str().split("#HEADER");
    let mut headers = String::new();
    let mut code = String::new();
    for (i, part) in parts.enumerate() {
        if i % 2 == 1 {
            headers += part;
        } else {
            code += part;
        }
    }
    macro_processor(headers, code, Vec::new())
}

fn macro_processor(
    headers: String,
    mut code: String,
    mut definitions: Vec<(String, String)>,
) -> Result<(String, Vec<(String, String)>), String> {
    for line in headers.as_str().split("\n") {
        let mut args = line.split(" ");
        loop {
            match args.next() {
                Some("") => {},
                Some("define") => {
                    match add_definition(String::from(args.next().expect("Invalid number of arguments for macro define")), String::from(args.next().expect("Invalid number of arguments for macro define")), definitions) {
                        Ok(res) => definitions = res,
                        Err(err) => return Err(err)
                    };
                },
                Some("import") => {
                    match import_file(String::from(args.next().expect("Invalid number of arguments for macro import")), code, definitions) {
                        Ok(res) => {
                            code = res.0;
                            definitions = res.1;
                        },
                        Err(err) => return Err(err)
                    };
                },
                Some(name) => {return Err(format!("Macro {} not recognised", name))},
                None => break
            }
        }
    }
    Ok((code, definitions))
}

fn import_file(
    path: String,
    mut code: String,
    mut definitions: Vec<(String, String)>,
) -> Result<(String, Vec<(String, String)>), String> {
    match read_file(path) {
        Ok(raw_code) => {
            return match preprocess(raw_code) {
                Ok(res) => {
                    code += &res.0;
                    definitions.extend(res.1);
                    Ok((code, definitions))
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

fn add_definition(
    name: String,
    value: String,
    mut definitions: Vec<(String, String)>,
) -> Result<Vec<(String, String)>, String> {
    definitions.push((name, value));
    Ok(definitions)
}
