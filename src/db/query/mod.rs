pub mod list;
pub mod task;
pub mod user;

const PAGE_LIMIT: i16 = 10;

fn OFFSET(page: i16) -> i16 {
    (page - 1) * PAGE_LIMIT
}
