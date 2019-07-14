use rss::Channel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry
{
    pub id: Option<i32>,
    pub title: String,
    pub link: String,
    pub comments: Option<String>,
    pub visited: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feed
{
    pub id: Option<i32>,
    pub url: String,
    pub title: String,
    pub description: String,
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OverviewContext
{
    pub feeds: Vec<Feed>,
}

impl Feed
{
    pub fn new(url: &str) -> Feed
    {
        let channel = Channel::from_url(url).unwrap();
        let entries = channel
            .items()
            .iter()
            .map(|item| Entry {
                id: None,
                title: String::from(item.title().unwrap()),
                link: String::from(item.link().unwrap()),
                comments: item.comments().map(String::from),
                visited: false,
            })
            .collect();

        Feed {
            id: None,
            url: String::from(url),
            title: String::from(channel.title()),
            description: String::from(channel.description()),
            entries,
        }
    }
}

impl OverviewContext
{
    pub fn new(urls: &[&str]) -> OverviewContext
    {
        let feeds: Vec<Feed> = urls.iter().map(|x| Feed::new(x)).collect();

        OverviewContext { feeds }
    }
}
