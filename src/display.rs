use failure::Error;
use reqwest;
use serde_json;
use theme;
use std::collections::BTreeMap;
use std::io::Write;

pub fn header(
    f: &mut Write,
    headers: &reqwest::header::Headers,
    t: &theme::Theme,
) -> Result<(), Error> {
    let mut hmap: BTreeMap<String, String> = BTreeMap::new();
    for h in headers.iter() {
        hmap.insert(h.name().to_string(), h.value_string());
    }

    for (name, value) in hmap {
        f.write_all(
            format!(
                "{}{} {}\n",
                t.header_name.paint(name),
                t.header_name.paint(":"),
                t.header_value.paint(value),
            ).as_bytes(),
        )?;
    }
    Ok(())
}

pub fn response_status(
    f: &mut Write,
    s: &reqwest::StatusCode,
    t: &theme::Theme,
) -> Result<(), Error> {
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

    f.write_all(
        format!(
            "{} {}\n",
            status_style.paint(format!("{}", s.as_u16())),
            t.status_message.paint(reason),
        ).as_bytes(),
    )?;
    Ok(())
}

pub fn request_path(f: &mut Write, req: &reqwest::Request, _t: &theme::Theme) -> Result<(), Error> {
    f.write_all(format!("Wee: {:?}\n", req).as_bytes())?;
    Ok(())
}

pub fn formatted_response(
    f: &mut Write,
    res: &mut reqwest::Response,
    t: &theme::Theme,
) -> Result<(), Error> {
    let thing = if let Some(ct) = res.headers().get::<reqwest::header::ContentType>() {
        match (ct.type_().as_str(), ct.subtype().as_str()) {
            ("application", "json") => json_display,
            _ => unformatted_response,
        }
    } else {
        unformatted_response
    };
    thing(f, res, t)
}

fn json_display(f: &mut Write, res: &mut reqwest::Response, t: &theme::Theme) -> Result<(), Error> {
    let v: serde_json::Value = res.json()?;
    f.write_all(b"\n")?;
    let indent: usize = 0;
    _recursive_display(f, &v, t, indent)?;
    f.write_all(b"\n\n")?;
    Ok(())
}

pub fn unformatted_response(
    f: &mut Write,
    res: &mut reqwest::Response,
    _: &theme::Theme,
) -> Result<(), Error> {
    f.write_all(b"\n")?;
    res.copy_to(f)?;
    Ok(())
}

fn _recursive_display(
    f: &mut Write,
    v: &serde_json::Value,
    t: &theme::Theme,
    indent: usize,
) -> Result<(), Error> {
    match *v {
        serde_json::Value::Array(ref a) => _display_array(f, a, t, indent)?,
        serde_json::Value::Bool(ref b) => {
            f.write_all(format!("{}", t.bool_value.paint(format!("{}", b))).as_bytes())?
        }
        serde_json::Value::Null => {
            f.write_all(format!("{}", t.null_value.paint("null")).as_bytes())?
        }
        serde_json::Value::Number(ref n) => {
            f.write_all(format!("{}", t.number_value.paint(format!("{}", n))).as_bytes())?
        }
        serde_json::Value::String(ref s) => {
            f.write_all(format!("\"{}\"", t.string_value.paint(format!("{}", s))).as_bytes())?
        }
        serde_json::Value::Object(ref o) => {
            if o.is_empty() {
                f.write_all(b"{}")?
            } else {
                _display_map(f, o, t, indent)?
            };
        }
    };

    Ok(())
}

fn _display_array(
    f: &mut Write,
    a: &[serde_json::Value],
    t: &theme::Theme,
    indent: usize,
) -> Result<(), Error> {
    f.write_all(b"[")?;
    for (i, element) in a.iter().enumerate() {
        let comma = if i > 0 { "," } else { "" };

        f.write_all(format!("{}\n{}", comma, " ".repeat(indent + 2)).as_bytes())?;

        match *element {
            serde_json::Value::Array(ref child) => {
                _display_array(f, child, t, indent + 2)?;
            }
            serde_json::Value::Bool(ref b) => {
                f.write_all(format!("{}", t.bool_value.paint(format!("{}", b))).as_bytes())?
            }
            serde_json::Value::Null => {
                f.write_all(format!("{}", t.null_value.paint("null")).as_bytes())?
            }
            serde_json::Value::Number(ref n) => {
                f.write_all(format!("{}", t.number_value.paint(format!("{}", n))).as_bytes())?
            }
            serde_json::Value::String(ref s) => {
                f.write_all(format!("\"{}\"", t.string_value.paint(format!("{}", s))).as_bytes())?
            }
            serde_json::Value::Object(ref o) => {
                _display_map(f, o, t, indent + 2)?;
            }
        }
    }
    f.write_all(format!("\n{}]", " ".repeat(indent)).as_bytes())?;
    Ok(())
}

fn _display_map(
    f: &mut Write,
    m: &serde_json::Map<String, serde_json::Value>,
    t: &theme::Theme,
    indent: usize,
) -> Result<(), Error> {
    f.write_all(b"{")?;
    for (i, (k, v)) in m.iter().enumerate() {
        let comma = if i > 0 { "," } else { "" };

        f.write_all(
            format!(
                "{}\n{}{}: ",
                comma,
                " ".repeat(indent + 2),
                t.key.paint(format!("\"{}\"", k))
            ).as_bytes(),
        )?;

        match *v {
            serde_json::Value::Array(ref a) => _display_array(f, a, t, indent + 2)?,
            serde_json::Value::Bool(ref b) => {
                f.write_all(format!("{}", t.bool_value.paint(format!("{}", b))).as_bytes())?
            }
            serde_json::Value::Null => {
                f.write_all(format!("{}", t.null_value.paint("null")).as_bytes())?
            }
            serde_json::Value::Number(ref n) => {
                f.write_all(format!("{}", t.number_value.paint(format!("{}", n))).as_bytes())?
            }
            serde_json::Value::String(ref s) => {
                f.write_all(format!("\"{}\"", t.string_value.paint(format!("{}", s))).as_bytes())?
            }
            serde_json::Value::Object(ref o) => {
                _display_map(f, o, t, indent + 2)?;
            }
        }
    }
    f.write_all(format!("\n{}}}", " ".repeat(indent)).as_bytes())?;
    Ok(())
}
