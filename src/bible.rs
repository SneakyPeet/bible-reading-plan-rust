use crate::books::*;

// TODO pull list from string or config or something

pub fn reading_list() -> ReadingList {
    ReadingList {
        booklists: vec![BookList::new(
            1,
            vec![
                Book::new("Genesis", 55), // TODO GET CORRECT
                Book::new("Exodus", 44),
            ],
        )],
    }
}
