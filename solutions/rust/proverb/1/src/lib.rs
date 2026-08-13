pub fn build_proverb(list: &[&str]) -> String {
    let mut res = String::new();
    (*list).windows(2).for_each(|item| {
        res.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            item[0], item[1],
        ))
    });

    if list.len() >= 1 {
        res.push_str(&format!("And all for the want of a {}.", list[0]));
    }

    res
}
