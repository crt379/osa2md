pub fn vec_clone_and_push<T>(s: &Vec<T>, v: T) -> Vec<T>
where
    T: Clone,
{
    let mut nvec = s.clone();
    nvec.push(v);
    nvec
}

pub fn vec_clone_and_pushs<T, I>(s: &Vec<T>, vs: I) -> Vec<T>
where
    T: Clone,
    I: IntoIterator<Item = T>,
{
    let mut nvec = s.clone();
    vs.into_iter().for_each(|v| nvec.push(v));
    nvec
}

pub fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}
