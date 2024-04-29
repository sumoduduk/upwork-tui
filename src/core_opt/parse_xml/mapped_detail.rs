use std::collections::HashMap;

use crate::JobPost;
use color_eyre::eyre::{self, eyre};
use scraper::{Html, Selector};

pub fn mapped_detail(
    posted: String,
    timestamp: i64,
    title_raw: String,
    link_raw: String,
    desc: String,
) -> eyre::Result<JobPost> {
    let links: Vec<_> = link_raw.split('?').collect();

    let details = get_detail(&desc)?;

    let category = details
        .get("Category")
        .ok_or_else(|| eyre!("category not found"))?;
    let result_string = category.to_lowercase().replace(' ', "_");

    let job_post = JobPost {
        title: title_raw,
        link: links[0].to_owned(),
        detail: details,
        posted_on: posted,
        posted_timestamp: timestamp,
        category: result_string,
    };

    Ok(job_post)
}

pub fn get_detail(description: &str) -> eyre::Result<HashMap<String, String>> {
    let doc = Html::parse_document(description);

    let selector = Selector::parse("b").unwrap();

    let elements = doc.select(&selector);
    let mut mapped = HashMap::new();

    for element in elements {
        let key = element.inner_html();

        if key == *"Posted On".to_owned() {
            continue;
        }

        let value: String = element
            .next_sibling()
            .ok_or_else(|| eyre!("html parse: dont have next sibling"))?
            .value()
            .as_text()
            .ok_or_else(|| eyre!("html parse: are not a text"))?
            .to_string();

        let value: Vec<&str> = value.trim_start_matches(':').split_whitespace().collect();
        let value = value.join(" ");

        mapped.insert(key, value);
    }

    let nodes = doc.tree.nodes();

    for node in nodes {
        let val = node.value();
        if val.is_element() {
            if val.as_element().unwrap().name() == "b" {
                break;
            }
        }

        if val.is_text() {
            dbg!(val.as_text().unwrap());
        }
    }

    Ok(mapped)
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use super::*;
    use std::collections::HashMap;

    fn string_to_datetime(input: &str) -> eyre::Result<DateTime<Utc>> {
        let date = DateTime::parse_from_rfc2822(input)?.with_timezone(&Utc);

        Ok(date)
    }

    #[test]
    fn test_extract_detail() {
        let test1 = "We are looking for a part-time designer who can transform our Board of Directors update presentation outline into a visually appealing presentation in either Figma or PowerPoint within one day.<br /><br />\nThis will be an iterative process as we anticipate requesting changes and updating the presentation outline. It&#039;s crucial that the selected designer is fluent in Russian and proficient in English, as there may be potential for longer-term collaboration in the future.<br /><br />\nThe design style should be minimalistic, similar to our other corporate presentations (Example and Logobook will be provided).<br /><br /><b>Budget</b>: $80\n<br /><b>Posted On</b>: April 29, 2024 09:00 UTC<br /><b>Category</b>: Presentation Design<br /><b>Skills</b>:Financial Presentation,     Marketing Presentation,     Sales Presentation,     Analytical Presentation,     Presentation Design,     Graphic Design,     Microsoft PowerPoint,     Business Presentation    \n<br /><b>Skills</b>:        Financial Presentation,                     Marketing Presentation,                     Sales Presentation,                     Analytical Presentation,                     Presentation Design,                     Graphic Design,                     Microsoft PowerPoint,                     Business Presentation            <br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Urgent-Transform-outline-into-presentation_%7E0136d9648f43b2532a?source=rss\">click to apply</a>\n";
        let res = get_detail(test1);

        assert!(res.is_ok());
    }

    #[test]
    fn test_1_get_detail() {
        let title_test = "Title 01".to_string();
        let link_test = "https://linktest.com".to_string();
        let posted = "Fri, 01 Sep 2023 02:19:13 +0000".to_string();
        let timestamp = 1693534753;
        let category = "web_design".to_string();

        let test1 = "Picture needs to be designed for the HERO page. Background needs to be changed and some design adjustments<br /><br /><b>Hourly Range</b>: $10.00-$20.00\n\n<br /><b>Posted On</b>: September 01, 2023 02:17 UTC<br /><b>Category</b>: Web Design<br /><b>Skills</b>:Web Design,     Graphic Design,     Illustration,     Website,     Landing Page,     Blog,     Website Asset    \n<br /><b>Skills</b>:        Web Design,                     Graphic Design,                     Illustration,                     Website,                     Landing Page,                     Blog,                     Website Asset            <br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Website-Hero-Page_%7E014431774d3a21a1a2?source=rss\">click to apply</a>\n";
        let mut expected1 = HashMap::new();
        expected1.insert("Hourly Range".to_string(), "$10.00-$20.00".to_string());
        expected1.insert("Category".to_string(), "Web Design".to_string());
        expected1.insert(
            "Skills".to_string(),
            "Web Design, Graphic Design, Illustration, Website, Landing Page, Blog, Website Asset"
                .to_string(),
        );
        expected1.insert("Country".to_string(), "United States".to_string());

        let job_post = JobPost {
            title: "Title 01".to_string(),
            link: "https://linktest.com".to_string(),
            detail: expected1,
            posted_on: posted.clone(),
            posted_timestamp: timestamp,
            category,
        };

        let mapped_detail =
            mapped_detail(posted, timestamp, title_test, link_test, test1.to_string()).unwrap();

        assert_eq!(job_post, mapped_detail);
        let time = string_to_datetime(&mapped_detail.posted_on);

        assert!(time.is_ok());
    }

    #[test]
    fn test_2_get_detail() {
        let title_test = "Title 02".to_string();
        let link_test = "https://linktest2.com".to_string();
        let posted = "Sat, 02 Sep 2023 03:19:13 +0000".to_string();
        let timestamp = 1693621153;
        let category = "web_design".to_string();

        let test2 = "We need a new design for our company website. Must be modern and user-friendly.<br /><br /><b>Budget</b>: $500\n\n<br /><b>Posted On</b>: September 02, 2023 03:17 UTC<br /><b>Category</b>: Web Design<br /><b>Skills</b>:Web Design,     Graphic Design,     User Experience Design,     Website,     Landing Page,     Blog,     Website Asset    \n<br /><b>Skills</b>:        Web Design,                     Graphic Design,                     User Experience Design,                     Website,                     Landing Page,                     Blog,                     Website Asset            <br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Website-Design_%7E014431774d3a21a1a3?source=rss\">click to apply</a>\n";
        let mut expected2 = HashMap::new();
        expected2.insert("Budget".to_string(), "$500".to_string());
        expected2.insert("Category".to_string(), "Web Design".to_string());
        expected2.insert(
        "Skills".to_string(),
        "Web Design, Graphic Design, User Experience Design, Website, Landing Page, Blog, Website Asset"
            .to_string(),
    );
        expected2.insert("Country".to_string(), "United States".to_string());

        let job_post = JobPost {
            title: title_test.clone(),
            link: link_test.clone(),
            detail: expected2,
            posted_on: posted.clone(),
            posted_timestamp: timestamp,
            category,
        };

        let mapped_detail = mapped_detail(
            posted,
            timestamp,
            title_test.clone(),
            link_test.clone(),
            test2.to_string(),
        )
        .unwrap();

        assert_eq!(job_post, mapped_detail);
        let time = string_to_datetime(&mapped_detail.posted_on);

        assert!(time.is_ok());
    }

    #[test]
    fn test_3_get_detail() {
        let title_test = "Title 03".to_string();
        let link_test = "https://linktest3.com".to_string();
        let posted = "Sun, 03 Sep 2023 04:19:13 +0000".to_string();
        let timestamp = 1693707553;
        let category = "graphic_design".to_string();

        let test3 = "We need a new logo for our company. Must be modern and eye-catching.<br /><br /><b>Budget</b>: $300\n\n<br /><b>Posted On</b>: September 03, 2023 04:17 UTC<br /><b>Category</b>: Graphic Design<br /><b>Skills</b>:Logo Design,     Graphic Design,     Branding   \n<br /><b>Skills</b>:        Logo Design,                     Graphic Design,                     Branding           <br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Logo-Design_%7E014431774d3a21a1a4?source=rss\">click to apply</a>\n";
        let mut expected3 = HashMap::new();
        expected3.insert("Budget".to_string(), "$300".to_string());
        expected3.insert("Category".to_string(), "Graphic Design".to_string());
        expected3.insert(
            "Skills".to_string(),
            "Logo Design, Graphic Design, Branding".to_string(),
        );
        expected3.insert("Country".to_string(), "United States".to_string());

        let job_post = JobPost {
            title: title_test.clone(),
            link: link_test.clone(),
            detail: expected3,
            posted_on: posted.clone(),
            posted_timestamp: timestamp,
            category,
        };

        let mapped_detail = mapped_detail(
            posted,
            timestamp,
            title_test.clone(),
            link_test.clone(),
            test3.to_string(),
        )
        .unwrap();

        assert_eq!(job_post, mapped_detail);
        let time = string_to_datetime(&mapped_detail.posted_on);

        assert!(time.is_ok());
    }

    #[test]
    fn test_4_get_detail() {
        let title_test = "Title 04".to_string();
        let link_test = "https://linktest4.com".to_string();
        let posted = "Mon, 04 Sep 2023 05:19:13 +0000".to_string();
        let timestamp = 1693793953;
        let category = "writing".to_string();

        let test4 = "We need a content writer for our company blog. Must have experience in the tech industry.<br /><br /><b>Budget</b>: $1000\n\n<br /><b>Posted On</b>: September 04, 2023 05:17 UTC<br /><b>Category</b>: Writing<br /><b>Skills</b>:Content Writing,     Blog Writing,     Tech Writing   \n<br /><b>Skills</b>:        Content Writing,                     Blog Writing,                     Tech Writing           <br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Content-Writer-Needed_%7E014431774d3a21a1a5?source=rss\">click to apply</a>\n";
        let mut expected4 = HashMap::new();
        expected4.insert("Budget".to_string(), "$1000".to_string());
        expected4.insert("Category".to_string(), "Writing".to_string());
        expected4.insert(
            "Skills".to_string(),
            "Content Writing, Blog Writing, Tech Writing".to_string(),
        );
        expected4.insert("Country".to_string(), "United States".to_string());

        let job_post = JobPost {
            title: title_test.clone(),
            link: link_test.clone(),
            detail: expected4,
            posted_on: posted.clone(),
            posted_timestamp: timestamp,
            category,
        };

        let mapped_detail = mapped_detail(
            posted,
            timestamp,
            title_test.clone(),
            link_test.clone(),
            test4.to_string(),
        )
        .unwrap();

        assert_eq!(job_post, mapped_detail);
    }

    #[test]
    fn test_5_get_detail() {
        let title_test = "Title 05".to_string();
        let link_test = "https://linktest5.com".to_string();
        let posted = "Tue, 05 Sep 2023 06:19:13 +0000".to_string();
        let timestamp = 1693880353;
        let category = "web_development".to_string();

        let test5 = "We need a web developer for our company website. Must have experience with modern web technologies.<br /><br /><b>Budget</b>: $5000\n\n<br /><b>Posted On</b>: September 05, 2023 06:17 UTC<br /><b>Category</b>: Web Development<br /><b>Skills</b>:HTML,     CSS,     JavaScript,     Web Development   \n<br /><b>Skills</b>:        HTML,                     CSS,                     JavaScript,                     Web Development           <br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Web-Developer-Needed_%7E014431774d3a21a1a6?source=rss\">click to apply</a>\n";
        let mut expected5 = HashMap::new();
        expected5.insert("Budget".to_string(), "$5000".to_string());
        expected5.insert("Category".to_string(), "Web Development".to_string());
        expected5.insert(
            "Skills".to_string(),
            "HTML, CSS, JavaScript, Web Development".to_string(),
        );
        expected5.insert("Country".to_string(), "United States".to_string());

        let job_post = JobPost {
            title: title_test.clone(),
            link: link_test.clone(),
            detail: expected5,
            posted_on: posted.clone(),
            posted_timestamp: timestamp,
            category,
        };

        let mapped_detail = mapped_detail(
            posted,
            timestamp,
            title_test.clone(),
            link_test.clone(),
            test5.to_string(),
        )
        .unwrap();

        assert_eq!(job_post, mapped_detail);
    }

    // #[test]
    // fn test_error_get_detail() {
    //     let title_test = "Title 05".to_string();
    //     let link_test = "https://linktest5.com".to_string();
    //     let posted = "Tue, 05 Sep 2023 06:19:13 +0000".to_string();
    //     let timestamp = 1693880353;
    //
    //     let test5 = "We need a web developer for our company website. Must have experience with modern web technologies.<br /><br /><b>Posted On</b>: September 05, 2023 06:17 UTC<br /><b>Category</b>: Web Development<br /><b>Skills</b>:HTML,     CSS,     JavaScript,     Web Development   \n<br /><b>Skills</b>:        HTML,                     CSS,                     JavaScript,                     Web Development           <br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Web-Developer-Needed_%7E014431774d3a21a1a6?source=rss\">click to apply</a>\n";
    //     let mut expected5 = HashMap::new();
    //     expected5.insert("Category".to_string(), "Web Development".to_string());
    //     expected5.insert(
    //         "Skills".to_string(),
    //         "HTML, CSS, JavaScript, Web Development".to_string(),
    //     );
    //     expected5.insert("Country".to_string(), "United States".to_string());
    //
    //     let mapped_detail = mapped_detail(
    //         posted,
    //         timestamp,
    //         title_test.clone(),
    //         link_test.clone(),
    //         test5.to_string(),
    //     );
    //
    //     assert!(mapped_detail.is_err());
    // }
}
