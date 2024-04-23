mod mapped_detail;

use crate::JobPost;
use chrono::DateTime;
use color_eyre::eyre;
use mapped_detail::mapped_detail;
use rss::Channel;
use std::io::BufRead;

pub fn parse_xml<R>(reader: R) -> eyre::Result<Vec<JobPost>>
where
    R: BufRead,
{
    let channel = Channel::read_from(reader)?;

    let items = channel.items;
    let len = items.len();

    let mut data: Vec<JobPost> = Vec::with_capacity(len);

    for item in items {
        let desc = item.description;

        match desc {
            Some(description) => {
                let title = item.title.unwrap_or_default();
                let link = item.link.unwrap_or_default();

                let posted_on = item.pub_date.unwrap_or_default();

                let timestamp = parse_date(&posted_on)?;

                let job_post = mapped_detail(posted_on, timestamp, title, link, description)?;

                data.push(job_post);
            }
            None => continue,
        }
    }

    Ok(data)
}

fn parse_date(date_str: &str) -> eyre::Result<i64> {
    let dt = DateTime::parse_from_str(date_str, "%a, %d %b %Y %H:%M:%S %z")?;
    Ok(dt.timestamp())
}
