use bytes::Bytes;
use color_eyre::eyre;

pub mod get_bytes;
mod parse_xml;

use parse_xml::parse_xml;

use crate::FinalPost;

pub fn populate_data(byte_data: Bytes) -> eyre::Result<Vec<FinalPost>> {
    let result_data = parse_xml(&byte_data[..])?;

    let list_job: Vec<FinalPost> = result_data
        .into_iter()
        .map(|j| {
            let budget = j.detail.get("Budget");
            let hourly = j.detail.get("Hourly Range");

            let mut price = "Unknown".to_string();

            match (budget, hourly) {
                (Some(b), None) => {
                    price = format!("Budget : {}", b);
                }
                (None, Some(h)) => {
                    price = format!("Hourly Range : {}", h);
                }
                (_, _) => (),
            }

            let desc = j
                .detail
                .get("details")
                .unwrap_or(&"No Description".to_owned())
                .clone();

            let response_json = FinalPost {
                title: j.title.clone(),
                link: j.link.clone(),
                detail: desc.to_owned(),
                price,
            };
            response_json
        })
        .collect();

    Ok(list_job)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use std::fs::File;
    use std::io::Read;
    use tests::get_bytes::req_bytes;

    fn load_xml_file(path: &str) -> eyre::Result<Bytes> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        let data_bytes = Bytes::from(data);
        Ok(data_bytes)
    }

    #[test]
    fn test_from_file() -> eyre::Result<()> {
        let bytes_data = load_xml_file("job.xml")?;
        let res = populate_data(bytes_data);
        dbg!(&res);

        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_populate_data() -> eyre::Result<()> {
        let bytes_data = req_bytes("design").await?;
        let res = populate_data(bytes_data);

        dbg!(&res);

        assert!(res.is_ok());

        Ok(())
    }
}
