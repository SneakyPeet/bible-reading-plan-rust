mod bible;
mod books;

fn main() {
    let list = &bible::reading_list().booklists[0];
    println!("{:#?}", books::caluclate_book_list_stat(list, 94));
}
