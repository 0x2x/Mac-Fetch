use std::fmt::format;

use reqwest;
use serde::Deserialize;



// Translate json towards model
/*
{
  "login": "0x2x",
  "id": 83850208,
  "node_id": "MDQ6VXNlcjgzODUwMjA4",
  "avatar_url": "https://avatars.githubusercontent.com/u/83850208?v=4",
  "gravatar_id": "",
  "url": "https://api.github.com/users/0x2x",
  "html_url": "https://github.com/0x2x",
  "followers_url": "https://api.github.com/users/0x2x/followers",
  "following_url": "https://api.github.com/users/0x2x/following{/other_user}",
  "gists_url": "https://api.github.com/users/0x2x/gists{/gist_id}",
  "starred_url": "https://api.github.com/users/0x2x/starred{/owner}{/repo}",
  "subscriptions_url": "https://api.github.com/users/0x2x/subscriptions",
  "organizations_url": "https://api.github.com/users/0x2x/orgs",
  "repos_url": "https://api.github.com/users/0x2x/repos",
  "events_url": "https://api.github.com/users/0x2x/events{/privacy}",
  "received_events_url": "https://api.github.com/users/0x2x/received_events",
  "type": "User",
  "user_view_type": "public",
  "site_admin": false,
  "name": "Nigel",
  "company": "@Microsoft",
  "blog": "",
  "location": "Seattle, WA",
  "email": null,
  "hireable": true,
  "bio": "💻\r\nI love using x y variables, if you can't tell.",
  "twitter_username": null,
  "public_repos": 43,
  "public_gists": 9,
  "followers": 7,
  "following": 8,
  "created_at": "2021-05-08T02:03:05Z",
  "updated_at": "2026-07-23T03:48:16Z"
}
*/
#[derive(Deserialize, Debug)]
pub struct User {
    id: u32,
    login: Option<String>,
    node_id: Option<String>,
    avatar_url: Option<String>,
    gravatar_id: Option<String>,
    url: Option<String>,
    html_url: Option<String>,
    followers_url: Option<String>,
    following_url: Option<String>,
    gists_url: Option<String>,
    starred_url: Option<String>,
    subscriptions_url: Option<String>,
    organizations_url: Option<String>,
    repos_url: Option<String>,
    events_url: Option<String>,
    received_events_url: Option<String>,
    r#type: Option<String>,
    user_view_type: Option<String>,
    site_admin: Option<bool>,
    name: Option<String>,
    company: Option<String>,
    blog: Option<String>,
    location: Option<String>,
    email: Option<String>,
    hireable: bool,
    bio: Option<String>,
    twitter_username: Option<String>,
    public_repos: u32,
    public_gists: u32,
    pub followers: u32,
    following: u32,
    created_at: Option<String>,
    updated_at: Option<String>

}

async fn get_profile_json(username: &str) -> Result<User, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/users/{}", username); // Assuming the user doesn't want to troll and wants to put a valid github username
    let user_get: User = client
        .get(&url)
        .header("User-Agent", "MacFetch")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .json()
        .await?;
    Ok(user_get)
}

pub async fn get_profile(username: &str) -> Result<User, Box<dyn std::error::Error>>  {
    let get_user = get_profile_json(username).await?;
    Ok(get_user)
}