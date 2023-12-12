#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Book {
    name: String,
    chapters: u8,
}

impl Book {
    fn new(name: &str, chapters: u8) -> Self {
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

#[derive(Eq, PartialEq, Debug)]
struct Chapter {
    book: Book,
    chapter_num: u8,
}

impl Chapter {
    fn new(book: &Book, chapter_num: u8) -> Self {
        Chapter {
            book: book.clone(),
            chapter_num,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
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

    fn test_book_a() -> Book {
        Book::new("A", 31)
    }

    fn test_book_b() -> Book {
        Book::new("B", 1)
    }

    fn test_book_c() -> Book {
        Book::new("C", 3)
    }

    fn test_book_list() -> BookList {
        BookList {
            i: 1,
            books: vec![test_book_a(), test_book_b(), test_book_c()],
        }
    }

    #[test]
    fn book_constructor() {
        let expected = Book {
            name: "A".to_string(),
            chapters: 31,
        };
        assert_eq!(expected, test_book_a());
    }

    #[test]
    fn caluclate_book_list_stat_given_day_1() {
        let given_day = 1;
        let expected_book = test_book_a();
        let expected_chapter = 1;
        let expected = BookListStat {
            chapter: Chapter::new(&expected_book, expected_chapter),
        };
        let given = test_book_list();
        assert_eq!(expected, caluclate_book_list_stat(&given, given_day))
    }

    #[test]
    fn caluclate_book_list_stat_with_single_chapter() {
        let given_day = 32;
        let expected_book = test_book_b();
        let expected_chapter = 1;
        let expected = BookListStat {
            chapter: Chapter::new(&expected_book, expected_chapter),
        };
        let given = test_book_list();
        assert_eq!(expected, caluclate_book_list_stat(&given, given_day))
    }

    #[test]
    fn caluclate_book_list_stat_given_day_last_book() {
        let given_day = 35;
        let expected_book = test_book_c();
        let expected_chapter = 3;
        let expected = BookListStat {
            chapter: Chapter::new(&expected_book, expected_chapter),
        };
        let given = test_book_list();
        assert_eq!(expected, caluclate_book_list_stat(&given, given_day))
    }

    #[test]
    fn caluclate_book_list_stat_wrap_once() {
        let given_day = 37;
        let expected_book = test_book_a();
        let expected_chapter = 2;
        let expected = BookListStat {
            chapter: Chapter::new(&expected_book, expected_chapter),
        };
        let given = test_book_list();
        assert_eq!(expected, caluclate_book_list_stat(&given, given_day))
    }

    #[test]
    fn caluclate_book_list_stat_wrap_twice() {
        let given_day = 67;
        let expected_book = test_book_b();
        let expected_chapter = 1;
        let expected = BookListStat {
            chapter: Chapter::new(&expected_book, expected_chapter),
        };
        let given = test_book_list();
        assert_eq!(expected, caluclate_book_list_stat(&given, given_day))
    }
}
