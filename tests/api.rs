use rand::distributions::{Alphanumeric, DistString};
use twitter_v2::{BearerToken, DraftTweet, TwitterApi};

fn get_api() -> TwitterApi<BearerToken> {
    TwitterApi::new(BearerToken::new(
        std::env::var("BEARER_TOKEN").expect("BEARER_TOKEN not found"),
    ))
}

fn rand_str(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

#[tokio::test]
async fn get_tweets() {
    let res = get_api()
        .get_tweets(&[1261326399320715264, 1278347468690915330], None, None)
        .await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert_eq!(res.unwrap().data.len(), 2);
}

#[tokio::test]
async fn get_tweet() {
    let res = get_api().get_tweet(1261326399320715264, None, None).await;
    assert!(res.is_ok(), "{}", res.unwrap_err())
}

async fn post_and_delete_tweet(tweet: &DraftTweet) {
    let api = get_api();
    let res = api.post_tweet(tweet).await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let id = res.unwrap().data.id;
    let res = api.delete_tweet(id).await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert!(res.unwrap().data.deleted)
}

#[tokio::test]
async fn manage_tweet() {
    post_and_delete_tweet(&DraftTweet {
        text: Some(rand_str(20)),
        ..Default::default()
    })
    .await;
}

#[tokio::test]
async fn get_users_me() {
    let res = get_api().get_users_me().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
}
