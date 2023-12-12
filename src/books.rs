#[derive(Eq, PartialEq, Debug)]
pub struct Book {
    name: String,
    chapters: u8,
}

impl Book {
    pub fn new(name: &str, chapters: u8) -> Self {
        Book {
            name: name.to_string(),
            chapters,
        }
    }
}

struct BookList {
    i: u8,
    books: Vec<Book>,
}

struct Chapter {
    book: Book,
    chapter_num: u8,
}

struct BookListStat {
    chapter: Chapter,
}

fn caluclate_book_list_stat(list: &BookList, day: u32) -> BookListStat {
    BookListStat {
        chapter: Chapter {
            book: Book {
                name: "TODO".to_string(),
                chapters: 1,
            },
            chapter_num: 1,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // const BOOK_1: Book = Book {
    //     name: "B1".to_string(),
    //     chapters: 55,
    // };
    //
    // const BOOK_2: Book = Book {
    //     name: "B2".to_string(),
    //     chapters: 1,
    // };
    //
    // const BOOK_3: Book = Book {
    //     name: "B3".to_string(),
    //     chapters: 3,
    // };
    //
    // const BOOK_LIST: BookList = BookList {
    //     books: vec![BOOK_1, BOOK_2, BOOK_3],
    //     i: 1,
    // };

    #[test]
    fn book_constructor() {
        let expected = Book {
            name: "A".to_string(),
            chapters: 10,
        };
        assert_eq!(expected, Book::new("A", 10));
    }
}
