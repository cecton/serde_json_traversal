#[macro_export]
macro_rules! serde_json_traversal {
    ($var:expr => [$index:expr] => $($tokens:tt)=>+) => {{
        let object = serde_json_traversal!($var => [$index]);

        serde_json_traversal!(with_result object => $($tokens)=>+)
    }};
    ($var:expr => $key:expr => $($tokens:tt)=>+) => {{
        let object = serde_json_traversal!($var => $key);

        serde_json_traversal!(with_result object => $($tokens)=>+)
    }};
    ($var:expr => [$index:expr]) => {{
        $var.as_array()
            .ok_or_else(|| format!("Not an array: {}", stringify!($var)))
            .and_then(|x| x.get($index)
                .ok_or_else(|| format!("Key {:?} not found.", stringify!($index))))
    }};
    ($var:expr => $key:expr) => {{
        $var.as_object()
            .ok_or_else(|| format!("Not an object: {}", stringify!($var)))
            .and_then(|x| x.get(stringify!($key))
                .ok_or_else(|| format!("Key {:?} not found.", stringify!($key))))
    }};
    (with_result $var:expr => [$index:expr] => $($tokens:tt)=>+) => {{
        let object = serde_json_traversal!(with_result $var => [$index]);

        serde_json_traversal!(with_result object => $($tokens)=>+)
    }};
    (with_result $var:expr => $key:expr => $($tokens:tt)=>+) => {{
        let object = serde_json_traversal!(with_result $var => $key);

        serde_json_traversal!(with_result object => $($tokens)=>+)
    }};
    (with_result $var:expr => [$index:expr]) => {{
        $var.and_then(|object| serde_json_traversal!(object => [$index]))
    }};
    (with_result $var:expr => $key:expr) => {{
        $var.and_then(|object| serde_json_traversal!(object => $key))
    }};
}
