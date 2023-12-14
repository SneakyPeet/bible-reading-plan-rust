#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Book {
    name: String,
    chapters: u32,
}

impl Book {
    pub fn new(name: &str, chapters: u32) -> Self {
        Book {
            name: name.to_string(),
            chapters,
        }
    }
}

pub struct BookList {
    pub i: u32,
    pub books: Vec<Book>,
}

impl BookList {
    pub fn new(i: u32, books: Vec<Book>) -> Self {
        Self { i, books }
    }
}

pub struct ReadingList {
    pub booklists: Vec<BookList>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Chapter {
    book: Book,
    chapter_num: u32,
}

impl Chapter {
    pub fn new(book: &Book, chapter_num: u32) -> Self {
        Chapter {
            book: book.clone(),
            chapter_num,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct BookListStat {
    chapter: Chapter,
    total_chapters: u32,
    chapter_index: u32,
}

fn calculate_list_lookup(book_list: &BookList) -> (u32, Vec<(u32, &Book)>) {
    let vec_size: u32 = book_list.books.iter().map(|x| x.chapters).sum();

    let mut lookup: Vec<(u32, &Book)> = Vec::with_capacity(vec_size as usize);
    for book in book_list.books.iter() {
        for chapter in 1..book.chapters + 1 {
            lookup.push((chapter, book))
        }
    }
    (vec_size, lookup)
}

pub fn caluclate_book_list_stat(list: &BookList, day: u32) -> BookListStat {
    let (lookup_size, lookup) = calculate_list_lookup(list);
    let chapter_index = (day - 1).rem_euclid(lookup_size);
    let (chapter_num, book) = lookup[chapter_index as usize];
    BookListStat {
        chapter: Chapter::new(book, chapter_num),
        total_chapters: lookup_size,
        chapter_index: chapter_index + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_constructor() {
        let expected = Book {
            name: "A".to_string(),
            chapters: 31,
        };
        assert_eq!(expected, test_book_a());
    }

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
    fn caluclate_book_list_stat_given_day_1() {
        let given_day = 1;
        let expected_book = test_book_a();
        let expected_chapter = 1;
        let expected = BookListStat {
            chapter: Chapter::new(&expected_book, expected_chapter),
            total_chapters: 35,
            chapter_index: 1,
        };
        let given = test_book_list();
        assert_eq!(expected, caluclate_book_list_stat(&given, given_day))
    }

    #[test]
    fn caluclate_book_list_stat_given_day_last_day_first_book() {
        let given_day = 31;
        let expected_book = test_book_a();
        let expected_chapter = 31;
        let expected = BookListStat {
            chapter: Chapter::new(&expected_book, expected_chapter),
            total_chapters: 35,
            chapter_index: 31,
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
            total_chapters: 35,
            chapter_index: 32,
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
            total_chapters: 35,
            chapter_index: 35,
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
            total_chapters: 35,
            chapter_index: 2,
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
            total_chapters: 35,
            chapter_index: 32,
        };
        let given = test_book_list();
        assert_eq!(expected, caluclate_book_list_stat(&given, given_day))
    }
}
