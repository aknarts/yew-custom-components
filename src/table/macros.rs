#[cfg(feature="table")]
#[macro_export]
macro_rules! column {
    ($a:expr) => {{
        $crate::components::table::types::Column {
            data_property: Some($a.to_string()),
            name: $a.to_string(),
            short_name: Some($a.to_string()),
            orderable: false,
            header_classes: vec![],
        }
    }};
    ($a:expr, $b:expr) => {{
        $crate::components::table::types::Column {
            data_property: Some($a.to_string()),
            name: $b.to_string(),
            short_name: Some($b.to_string()),
            orderable: false,
            header_classes: vec![],
        }
    }};
    ($a:expr, $b:expr, $c:expr) => {
        $crate::components::table::types::Column {
            data_property: Some($a.to_string()),
            name: $b.to_string(),
            short_name: Some($c.to_string()),
            orderable: false,
            header_classes: vec![],
        }
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::components::table::types::Column {
            data_property: Some($a.to_string()),
            name: $b.to_string(),
            short_name: Some($c.to_string()),
            orderable: $d,
            header_classes: vec![],
        }
    };
}

#[cfg(feature="table")]
#[macro_export]
macro_rules! columns {
( $( ( $($args:expr),* ) )+ ) => {
    vec![$(
        $crate::column![$($args),*]
    ),+]
};
}
