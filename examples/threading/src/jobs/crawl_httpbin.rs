extern crate reqwest;
extern crate serde_json;

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use reqwest::blocking as request;
use reqwest::StatusCode;
use serde_json as json;

fn fetch_response(tx: mpsc::Sender<Result<Box<json::Value>, bool>>) {
  let begin = Instant::now();
  let res: request::Response = request::get("http://httpbin.org/get").unwrap();
  if res.status() != StatusCode::OK {
    tx.send(Err(false)).unwrap();
  }

  let v = res.json::<HashMap<String, json::Value>>().unwrap();
  let amz = v
    .get("headers")
    .unwrap()
    .as_object()
    .unwrap()
    .get("X-Amzn-Trace-Id")
    .unwrap();
  tx.send(Ok(Box::new(amz.clone()))).unwrap();
  println!(
    "[{:?}] Job done; time taken: {:?}",
    thread::current().name().unwrap(),
    begin.elapsed()
  );
}

pub fn crawl() {
  let mut v = Vec::new();
  let begin = Instant::now();

  let (tx, rx) = mpsc::channel();
  for i in 1..10 {
    let txc = tx.clone();
    let t = thread::Builder::new()
      .name(format!("thread-{}", i))
      .spawn(move || fetch_response(txc))
      .unwrap();
    v.push(t);
  }

  // No need to join as we are waiting for their results in the queue
  // for t in v {
  //   t.join().unwrap();
  // }

  for _ in 1..10 {
    let _ = rx.recv().unwrap();
  }

  println!("Total time taken for crawl: {:?}", begin.elapsed());
}
