# wson

[JSON](https://www.json.org/json-en.html) parser made with [nom](https://docs.rs/nom/latest/nom/).

## Usage
``` rust
let value = parse(
    "{\"menu\": {
       \"id\": \"file\",
       \"value\": \"File\",
       \"popup\": {
         \"menuitem\": [
           {\"value\": \"New\", \"onclick\": \"CreateNewDoc()\"},
           {\"value\": \"Open\", \"onclick\": \"OpenDoc()\"},
           {\"value\": \"Close\", \"onclick\": \"CloseDoc()\"}
         ]
       }
    }}",
)?;
let expected = Value::Object(HashMap::from([(
    "menu".to_string(),
    Value::Object(HashMap::from([
        ("id".to_string(), Value::String("file".to_string())),
        ("value".to_string(), Value::String("File".to_string())),
        (
            "popup".to_string(),
            Value::Object(HashMap::from([(
                "menuitem".to_string(),
                Value::Array(vec![
                    Value::Object(HashMap::from([
                        ("value".to_string(), Value::String("New".to_string())),
                        (
                            "onclick".to_string(),
                            Value::String("CreateNewDoc()".to_string()),
                        ),
                    ])),
                    Value::Object(HashMap::from([
                        ("value".to_string(), Value::String("Open".to_string())),
                        (
                            "onclick".to_string(),
                            Value::String("OpenDoc()".to_string()),
                        ),
                    ])),
                    Value::Object(HashMap::from([
                        ("value".to_string(), Value::String("Close".to_string())),
                        (
                            "onclick".to_string(),
                            Value::String("CloseDoc()".to_string()),
                        ),
                    ])),
                ]),
            )])),
        ),
    ])),
)]));
assert_eq!(value, expected);
```


# Installation

``` toml
[dependencies]
wson = "*"
```
