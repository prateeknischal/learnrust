mod jobs;

use jobs::{crawl_httpbin, random_wait};

fn main() {
    random_wait::random_wait();
    crawl_httpbin::crawl();
}
