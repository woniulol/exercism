use std::cmp::max;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.len() > second_list.len() {
        for i in 0..first_list.len() {
            if i + second_list.len() > first_list.len() {
                return Comparison::Unequal;
            }
            if first_list[i..i + second_list.len()] == *second_list {
                return Comparison::Superlist;
            }
        }
    } else {
        for i in 0..second_list.len() {
            if i + first_list.len() > second_list.len() {
                return Comparison::Unequal;
            }
            if second_list[i..i + first_list.len()] == *first_list {
                return Comparison::Sublist;
            }
        }
    }

    if first_list == second_list {
        return Comparison::Equal;
    }

    Comparison::Unequal
}
