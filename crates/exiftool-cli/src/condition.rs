//! Condition evaluation for -if option.
//!
//! # Why
//!
//! `-if "Make eq Canon"` filters which files to process. Supports eq, ne, gt, lt,
//! contains, startswith, endswith, and existence check.
//!
//! # Where used
//!
//! Read and rename loops — skip files that don't match the condition.

use exiftool_attrs::AttrValue;
use exiftool_formats::Metadata;

/// Condition operator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CondOp {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    Contains,
    StartsWith,
    EndsWith,
    Exists,
}

impl CondOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "eq" | "=" | "==" => Some(CondOp::Eq),
            "ne" | "!=" | "<>" => Some(CondOp::Ne),
            "gt" | ">" => Some(CondOp::Gt),
            "lt" | "<" => Some(CondOp::Lt),
            "ge" | ">=" => Some(CondOp::Ge),
            "le" | "<=" => Some(CondOp::Le),
            "contains" | "~" => Some(CondOp::Contains),
            "startswith" | "starts" => Some(CondOp::StartsWith),
            "endswith" | "ends" => Some(CondOp::EndsWith),
            _ => None,
        }
    }
}

/// Parsed condition.
pub struct Condition {
    pub tag: String,
    pub op: CondOp,
    pub value: String,
}

impl Condition {
    /// Parse condition string: "Tag op Value" or just "Tag" for existence check.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts.len() {
            1 => Some(Condition {
                tag: parts[0].to_string(),
                op: CondOp::Exists,
                value: String::new(),
            }),
            n if n >= 3 => {
                let tag = parts[0].to_string();
                let op = CondOp::from_str(parts[1])?;
                let value = parts[2..].join(" ");
                Some(Condition { tag, op, value })
            }
            _ => None,
        }
    }

    /// Evaluate condition against metadata.
    pub fn eval(&self, metadata: &Metadata) -> bool {
        let tag_value = metadata.exif.get(&self.tag)
            .map(|v| format_attr_value(v))
            .unwrap_or_default();

        if self.op == CondOp::Exists {
            return metadata.exif.contains(&self.tag);
        }

        if tag_value.is_empty() {
            return false;
        }

        let tag_num = parse_number(&tag_value);
        let val_num = parse_number(&self.value);

        match self.op {
            CondOp::Eq => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    (tn - vn).abs() < 0.001
                } else {
                    tag_value.eq_ignore_ascii_case(&self.value)
                }
            }
            CondOp::Ne => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    (tn - vn).abs() >= 0.001
                } else {
                    !tag_value.eq_ignore_ascii_case(&self.value)
                }
            }
            CondOp::Gt => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    tn > vn
                } else {
                    tag_value > self.value
                }
            }
            CondOp::Lt => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    tn < vn
                } else {
                    tag_value < self.value
                }
            }
            CondOp::Ge => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    tn >= vn
                } else {
                    tag_value >= self.value
                }
            }
            CondOp::Le => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    tn <= vn
                } else {
                    tag_value <= self.value
                }
            }
            CondOp::Contains => {
                tag_value.to_lowercase().contains(&self.value.to_lowercase())
            }
            CondOp::StartsWith => {
                tag_value.to_lowercase().starts_with(&self.value.to_lowercase())
            }
            CondOp::EndsWith => {
                tag_value.to_lowercase().ends_with(&self.value.to_lowercase())
            }
            CondOp::Exists => unreachable!(),
        }
    }
}

/// Parse number from string (handles ratios like "1/200", "f/2.8", "100 mm").
pub fn parse_number(s: &str) -> Option<f64> {
    let s = s.trim();
    let s = s.trim_end_matches(|c: char| c.is_alphabetic() || c == ' ');
    let s = s.strip_prefix("f/").unwrap_or(s);
    let s = s.strip_prefix("F/").unwrap_or(s);

    if let Some((num, den)) = s.split_once('/') {
        let n: f64 = num.trim().parse().ok()?;
        let d: f64 = den.trim().parse().ok()?;
        if d != 0.0 {
            return Some(n / d);
        }
    }

    s.parse().ok()
}

/// Format AttrValue for display/comparison.
pub fn format_attr_value(v: &AttrValue) -> String {
    match v {
        AttrValue::Str(s) => s.clone(),
        AttrValue::Bool(b) => b.to_string(),
        AttrValue::Int8(n) => n.to_string(),
        AttrValue::Int(n) => n.to_string(),
        AttrValue::UInt(n) => n.to_string(),
        AttrValue::Int64(n) => n.to_string(),
        AttrValue::UInt64(n) => n.to_string(),
        AttrValue::Float(f) => format!("{}", f),
        AttrValue::Double(f) => format!("{}", f),
        AttrValue::Rational(n, d) => {
            if *d == 1 { n.to_string() } else { format!("{}/{}", n, d) }
        }
        AttrValue::URational(n, d) => {
            if *d == 1 { n.to_string() } else { format!("{}/{}", n, d) }
        }
        AttrValue::Bytes(b) => format!("({} bytes)", b.len()),
        AttrValue::DateTime(dt) => dt.to_string(),
        AttrValue::Uuid(u) => u.to_string(),
        AttrValue::Json(j) => j.clone(),
        AttrValue::Vec3(v) => format!("{},{},{}", v[0], v[1], v[2]),
        AttrValue::Vec4(v) => format!("{},{},{},{}", v[0], v[1], v[2], v[3]),
        AttrValue::List(l) => format!("[{} items]", l.len()),
        AttrValue::Map(m) => format!("{{{}}} items", m.len()),
        AttrValue::Set(s) => format!("{{{} items}}", s.len()),
        AttrValue::Group(g) => format!("({} attrs)", g.len()),
    }
}

/// Check if file matches -if condition.
pub fn matches_condition(metadata: &Metadata, condition: &str) -> bool {
    if let Some(cond) = Condition::parse(condition) {
        cond.eval(metadata)
    } else {
        eprintln!("Warning: invalid condition syntax: {}", condition);
        true
    }
}
