use reqwest;
use serde_json;
use theme;
use std::collections::BTreeMap;
use std::io::{self, Write};

pub fn header(res: &reqwest::Response, t: &theme::Theme) {
    println!("{}", status_of(&res, t));

    let mut headers: BTreeMap<String, String> = BTreeMap::new();
    for h in res.headers().iter() {
        headers.insert(h.name().to_string(), h.value_string());
    }

    for (name, value) in headers {
        println!(
            "{}{} {}",
            t.header_name.paint(name),
            t.header_name.paint(":"),
            t.header_value.paint(value),
        );
    }
}

fn status_of(res: &reqwest::Response, t: &theme::Theme) -> String {
    let s = res.status();
    let status_style = if s.is_informational() || s.is_redirection() {
        t.status_info
    } else if s.is_success() {
        t.status_success
    } else {
        t.status_error
    };

    let reason = match s.canonical_reason() {
        Some(msg) => msg,
        None => "",
    };

    format!(
        "{} {}",
        status_style.paint(format!("{}", res.status().as_u16())),
        t.status_message.paint(reason),
    )
}

pub fn json(res: &mut reqwest::Response, t: &theme::Theme) {
    let v: serde_json::Value = res.json().expect("non-json-response");
    let mut stdout = io::stdout();
    stdout
        .write_all(b"\n")
        .expect("failed to write starting newline");
    let indent: usize = 0;
    _recursive_display(&mut stdout, &v, t, indent);

    stdout
        .write_all(b"\n\n")
        .expect("failed to write trailing newlines");
}

fn _recursive_display(f: &mut Write, v: &serde_json::Value, t: &theme::Theme, indent: usize) {
    match *v {
        serde_json::Value::Array(ref a) => _display_array(f, a, t, indent),
        serde_json::Value::Bool(ref b) => {
            f.write_all(format!("{}", t.bool_value.paint(format!("{}", b))).as_bytes())
                .expect("Failed to write bool");
        }
        serde_json::Value::Null => {
            f.write_all(format!("{}", t.null_value.paint("null")).as_bytes())
                .expect("Failed to write null");
        }
        serde_json::Value::Number(ref n) => {
            f.write_all(format!("{}", t.number_value.paint(format!("{}", n))).as_bytes())
                .expect("Failed to write number");
        }
        serde_json::Value::String(ref s) => {
            f.write_all(format!("{}", t.string_value.paint(format!("{}", s))).as_bytes())
                .expect("Failed to write string");
        }
        serde_json::Value::Object(ref o) => {
            if o.is_empty() {
                f.write_all(b"{}").expect("Failed to write emtpy object");
            } else {
                _display_map(f, o, t, indent);
            };
        }
    };
}

fn _display_array(f: &mut Write, a: &[serde_json::Value], t: &theme::Theme, indent: usize) {
    f.write_all(b"[").expect("Failed to write open of array");
    for (i, element) in a.iter().enumerate() {
        let comma = if i > 0 { "," } else { "" };

        f.write_all(format!("{}\n{}", comma, " ".repeat(indent + 2)).as_bytes())
            .expect("failed to write element separator");

        match *element {
            serde_json::Value::Array(ref child) => {
                _display_array(f, child, t, indent + 2);
            }
            serde_json::Value::Bool(ref b) => {
                f.write_all(format!("{}", t.bool_value.paint(format!("{}", b))).as_bytes())
                    .expect("Failed to write bool");
            }
            serde_json::Value::Null => {
                f.write_all(format!("{}", t.null_value.paint("null")).as_bytes())
                    .expect("Failed to write null");
            }
            serde_json::Value::Number(ref n) => {
                f.write_all(format!("{}", t.number_value.paint(format!("{}", n))).as_bytes())
                    .expect("Failed to write number");
            }
            serde_json::Value::String(ref s) => {
                f.write_all(format!("{}", t.string_value.paint(format!("{}", s))).as_bytes())
                    .expect("Failed to write string");
            }
            serde_json::Value::Object(ref o) => {
                _display_map(f, o, t, indent + 2);
            }
        }
    }
    f.write_all(format!("\n{}]", " ".repeat(indent)).as_bytes())
        .expect("Failed to write close of array");
}

fn _display_map(
    f: &mut Write,
    m: &serde_json::Map<String, serde_json::Value>,
    t: &theme::Theme,
    indent: usize,
) {
    f.write_all(b"{").expect("Failed to write start of object");
    for (i, (k, v)) in m.iter().enumerate() {
        let comma = if i > 0 { "," } else { "" };

        f.write_all(
            format!(
                "{}\n{}{}: ",
                comma,
                " ".repeat(indent + 2),
                t.key.paint(format!("\"{}\"", k))
            ).as_bytes(),
        ).expect("failed to write key");

        match *v {
            serde_json::Value::Array(ref a) => _display_array(f, a, t, indent + 2),
            serde_json::Value::Bool(ref b) => {
                f.write_all(format!("{}", t.bool_value.paint(format!("{}", b))).as_bytes())
                    .expect("Failed to write bool");
            }
            serde_json::Value::Null => {
                f.write_all(format!("{}", t.null_value.paint("null")).as_bytes())
                    .expect("Failed to write null");
            }
            serde_json::Value::Number(ref n) => {
                f.write_all(format!("{}", t.number_value.paint(format!("{}", n))).as_bytes())
                    .expect("Failed to write number");
            }
            serde_json::Value::String(ref s) => {
                f.write_all(format!("{}", t.string_value.paint(format!("{}", s))).as_bytes())
                    .expect("Failed to write string");
            }
            serde_json::Value::Object(ref o) => {
                _display_map(f, o, t, indent + 2);
            }
        }
    }
    f.write_all(format!("\n{}}}", " ".repeat(indent)).as_bytes())
        .expect("Failed to write close of object");
}
