use diesel::prelude::*;

sql_function!(fn random() -> Text);
