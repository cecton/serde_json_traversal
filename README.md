Synopsis
========

```rust
use serde_json_traversal::serde_json_traversal;

#[test]
fn success() {
    let json_obj = serde_json::json!({
        "foo": "bar",
        "bar": [
            "foo",
            "baz",
        ],
        "baz": {
            "foo": [
                "bar",
                {
                    "bar": "baz",
                },
            ],
        },
    });

   assert_eq!(serde_json_traversal!(json_obj => foo).unwrap(), "bar");
   assert_eq!(serde_json_traversal!(json_obj => bar => [0]).unwrap(), "foo");
   assert_eq!(serde_json_traversal!(json_obj => baz => foo => [0]).unwrap(), "bar");
   assert_eq!(serde_json_traversal!(json_obj => baz => foo => [1] => bar).unwrap(), "baz");

   let json_arr = serde_json::json!([
        "foo",
        {
            "bar": "baz",
        }
   ]);

   assert_eq!(serde_json_traversal!(json_arr => [0]).unwrap(), "foo");
   assert_eq!(serde_json_traversal!(json_arr => [1] => bar).unwrap(), "baz");
}
```
