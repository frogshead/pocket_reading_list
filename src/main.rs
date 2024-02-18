use getpocket::{list::ListExt, GetPocket};
use std::{collections::btree_map::IterMut, env, thread, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

let pocket = init_pocket_client().await;
 




let mut items = pocket.unwrap().list_of_items_paginate(0,100).await.unwrap();
println!("last time fetched: {:?}", items.since);
let items_count = items.list.len();
    println!("Articles saved: {:?}", items_count);
    let first = items.list.first_entry().unwrap();
    println!("first entry:\n{:?}", first);

    for item in items.list {
        println!("url: {:?}\n", item);
    }
    Ok(())
    

    
}


async fn init_pocket_client() -> Result<GetPocket, Box<dyn std::error::Error>>{
    let consumer_key = env::var("POCKET_CONSUMER_KEY").expect("POCKET_CONSUMER_KEY env needs to be set");
    let redirect_uri = env::var("POCKET_REDIRECT_URI").expect("POCKET_REDIRECT_URI env needs to be set");

    let pocket: Option<GetPocket>;

    match env::var("POCKET_ACCESS_TOKEN") {
    Ok(token) => {pocket =  Some(GetPocket::new(consumer_key, redirect_uri, token).await.unwrap());},
    Err(_) => {
        pocket = Some(GetPocket::init(consumer_key, redirect_uri, |token|{
            println!("token: {:?}",token);
            env::set_var("POCKET_ACCESS_TOKEN", token);
        }, |url|{
            println!("open url: {:?}", url); 
            let wait_time = time::Duration::from_secs(15);
            thread::sleep(wait_time);
            Ok(true)
        }).await.unwrap());

    },
}
    Ok(pocket.unwrap())
}