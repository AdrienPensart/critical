pub fn vec_option_to_vec(v: Vec<Option<String>>) -> Vec<String> {
    v
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
}

