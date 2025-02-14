use std::collections::BTreeMap;

struct Priority;

#[allow(dead_code)]
impl Priority {
    fn lowest() -> u8 {
        26
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Simple {
    pub subject: String,
    #[cfg_attr(feature = "serde-support", serde(default = "Priority::lowest"))]
    pub priority: u8,
    pub create_date: Option<crate::Date>,
    pub finish_date: Option<crate::Date>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub finished: bool,
    pub threshold_date: Option<crate::Date>,
    pub due_date: Option<crate::Date>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub contexts: Vec<String>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub projects: Vec<String>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub hashtags: Vec<String>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub tags: BTreeMap<String, String>,
}

impl Default for Simple {
    fn default() -> Self {
        Self {
            subject: String::new(),
            priority: 26,
            create_date: None,
            finish_date: None,
            finished: false,
            threshold_date: None,
            due_date: None,
            contexts: Vec::new(),
            projects: Vec::new(),
            hashtags: Vec::new(),
            tags: BTreeMap::new(),
        }
    }
}

impl std::str::FromStr for Simple {
    type Err = ();

    fn from_str(s: &str) -> Result<Simple, ()> {
        crate::parser::task(&s.to_owned())
    }
}

impl std::fmt::Display for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.finished {
            f.write_str("x ")?;
        }

        if self.priority < 26 {
            let priority = (b'A' + self.priority) as char;

            f.write_str(format!("({}) ", priority).as_str())?;
        }

        if let Some(finish_date) = self.finish_date {
            f.write_str(format!("{} ", finish_date.format("%Y-%m-%d")).as_str())?;
        }

        if let Some(create_date) = self.create_date {
            f.write_str(format!("{} ", create_date.format("%Y-%m-%d")).as_str())?;
        }

        f.write_str(self.subject.as_str())?;

        if let Some(due_date) = self.due_date {
            f.write_str(format!(" due:{}", due_date.format("%Y-%m-%d")).as_str())?;
        }

        if let Some(threshold_date) = self.threshold_date {
            f.write_str(format!(" t:{}", threshold_date.format("%Y-%m-%d")).as_str())?;
        }

        for (key, value) in &self.tags {
            f.write_str(format!(" {}:{}", key, value).as_str())?;
        }

        Ok(())
    }
}
