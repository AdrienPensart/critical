use itertools::Itertools;
use num_traits::ToPrimitive;
use std::{collections::HashSet, hash::Hash, iter::zip};

use crate::music::errors::CriticalErrorKind;

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[must_use]
pub fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| s.starts_with('.'))
}

pub async fn public_ip() -> Result<String, CriticalErrorKind> {
    let client = reqwest::Client::new();
    let response = client.head("https://www.wikipedia.org").send().await?;
    let header = "X-Client-IP";
    if let Some(ip) = response.headers().get(header) {
        match ip.to_str() {
            Ok(ip) => Ok(ip.to_string()),
            Err(e) => Err(CriticalErrorKind::HeaderError(e)),
        }
    } else {
        Err(CriticalErrorKind::NoPublicIp)
    }
}

pub fn interleave_evenly<T>(mut iterables: Vec<Vec<T>>) -> Result<Vec<T>, CriticalErrorKind>
where
    T: std::clone::Clone,
{
    let lengths = iterables.iter().map(Vec::len).collect::<Vec<usize>>();
    let dims = lengths.len();
    let lengths_permute = (0..dims)
        .sorted_unstable_by(|a, b| iterables[*a].len().cmp(&iterables[*b].len()))
        .rev()
        .collect::<Vec<usize>>();

    let mut lengths_desc = lengths_permute
        .iter()
        .map(|l| lengths[*l])
        .collect::<Vec<usize>>();

    let mut iters_desc = (0..lengths_desc.len()).collect::<Vec<usize>>();
    let delta_primary = lengths_desc.remove(0);
    iters_desc.remove(0);

    let mut errors = vec![
        (delta_primary
            .to_f64()
            .ok_or(CriticalErrorKind::InterleaveError)?
            / dims.to_f64().ok_or(CriticalErrorKind::InterleaveError)?)
        .floor()
        .to_i64()
        .ok_or(CriticalErrorKind::InterleaveError)?;
        lengths_desc.len()
    ];

    let mut to_yield: usize = lengths.iter().sum();
    let mut elements: Vec<T> = Vec::new();

    while to_yield > 0 {
        if !iterables[0].is_empty() {
            let next_elem = iterables[0].remove(0);
            elements.push(next_elem);
        }

        to_yield -= 1;
        errors = zip(&errors, &lengths_desc)
            .map(|(e, delta)| {
                let delta = (*delta).to_i64().map_or(0, |d| d);
                e - delta
            })
            .collect();

        for i in 0..errors.len() {
            if errors[i] < 0 {
                if !iterables[iters_desc[i]].is_empty() {
                    let next_elem = iterables[iters_desc[i]].remove(0);
                    elements.push(next_elem);
                }
                to_yield -= 1;
                errors[i] += delta_primary
                    .to_i64()
                    .ok_or(CriticalErrorKind::InterleaveError)?;
            }
        }
    }
    Ok(elements)
}

#[test]
fn interleave_evenly_tests() {
    let iterables = vec![vec![1, 3, 5, 7], vec![0, 2, 4, 6]];
    let result = interleave_evenly(iterables);
    assert_eq!(vec![1, 0, 3, 2, 5, 4, 7, 6], result.unwrap());

    let iterables = vec![vec![0, 1, 2, 3], vec![11, 12]];
    let result = interleave_evenly(iterables);
    assert_eq!(vec![0, 1, 11, 2, 3, 12], result.unwrap());
}
