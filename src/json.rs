use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::multispace0,
    combinator::{map, value},
    error::context,
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Null,
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}

fn parse_string(input: &str) -> IResult<&str, &str> {
    context(
        "string",
        delimited(tag("\""), take_till(|c| c == '"'), tag("\"")),
    )(input)
}

fn parse_bool(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

fn parse_null(input: &str) -> IResult<&str, JsonValue> {
    value(JsonValue::Null, tag("null"))(input)
}

fn parse_array(input: &str) -> IResult<&str, Vec<JsonValue>> {
    context(
        "array",
        delimited(
            tag("["),
            separated_list0(tag(","), delimited(multispace0, parse, multispace0)),
            tag("]"),
        ),
    )(input)
}

fn parse_object(input: &str) -> IResult<&str, HashMap<String, JsonValue>> {
    let parse_pair = separated_pair(
        delimited(multispace0, parse_string, multispace0),
        tag(":"),
        delimited(multispace0, parse, multispace0),
    );
    context(
        "object",
        delimited(
            tag("{"),
            map(
                separated_list0(tag(","), parse_pair),
                |pairs: Vec<(&str, JsonValue)>| {
                    let mut map = HashMap::new();
                    for (k, v) in pairs {
                        map.insert(k.to_string(), v);
                    }
                    map
                },
            ),
            tag("}"),
        ),
    )(input)
}

pub fn parse(input: &str) -> IResult<&str, JsonValue> {
    context(
        "parse",
        delimited(
            multispace0,
            alt((
                map(parse_object, |x| JsonValue::Object(x)),
                map(parse_array, |x| JsonValue::Array(x)),
                map(double, |x| JsonValue::Number(x)),
                map(parse_string, |s| JsonValue::Str(s.to_string())),
                map(parse_bool, |x| JsonValue::Bool(x)),
                parse_null,
            )),
            multispace0,
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stirng() {
        assert_eq!(
            parse_string(r#""1234 asdf" jjjj"#),
            Ok((" jjjj", "1234 asdf"))
        );
    }

    #[test]
    fn test_bool() {
        assert_eq!(parse_bool("true"), Ok(("", true)));
        assert_eq!(parse_bool("falseff"), Ok(("ff", false)));
    }

    #[test]
    fn test_array() {
        assert_eq!(
            parse_array("[123.4]"),
            Ok(("", vec![JsonValue::Number(123.4)]))
        );
        assert_eq!(
            parse_array(r#"[ 333,  "wow"   ,null  ]"#),
            Ok((
                "",
                vec![
                    JsonValue::Number(333.),
                    JsonValue::Str("wow".to_string()),
                    JsonValue::Null
                ]
            ))
        );
    }

    #[test]
    fn test_nested() {
        assert_eq!(
            parse_array(r#"[1, [2, [3], true, "[not an array]"], false]"#),
            Ok((
                "",
                vec![
                    JsonValue::Number(1.),
                    JsonValue::Array(vec![
                        JsonValue::Number(2.),
                        JsonValue::Array(vec![JsonValue::Number(3.)]),
                        JsonValue::Bool(true),
                        JsonValue::Str("[not an array]".to_string()),
                    ]),
                    JsonValue::Bool(false),
                ]
            ))
        )
    }
}
