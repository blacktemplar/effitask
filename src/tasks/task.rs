#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    inner: ::todo_txt::task::Extended,
    pub id: usize,
}

impl Task {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
            id: 0,
        }
    }

    pub fn markup_subject(&self) -> String {
        let mut subject = Self::markup_escape(&self.subject);

        let regex = ::regex::Regex::new(r"(?P<url>[\w]+://[^\s]+)").unwrap();
        subject = regex
            .replace_all(&subject, |caps: &::regex::Captures| {
                format!(
                    "<a href=\"{url}\">{url}</a>",
                    url = caps[1].replace("&", "&amp;")
                )
            })
            .into_owned();

        let regex = ::regex::Regex::new(r"(?P<space>^|[\s])(?P<tag>[\+@][\w\-\\]+)").unwrap();
        subject = regex
            .replace_all(&subject, "$space<b>$tag</b>")
            .into_owned();

        subject
    }

    /* TODO: use `glib:functions:markup_escape_text` */
    fn markup_escape(text: &str) -> String {
        text.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("'", "&apos;")
            .replace("\"", "&quot;")
    }
}

impl ::std::str::FromStr for Task {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let inner = ::todo_txt::task::Extended::from_str(s)?;

        Ok(Self {
            inner,
            id: 0,
        })
    }
}

impl ::std::ops::Deref for Task {
    type Target = ::todo_txt::task::Extended;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ::std::ops::DerefMut for Task {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ::std::fmt::Display for Task {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use std::ops::Deref;

        f.write_str(format!("{}", self.deref()).as_str())?;

        Ok(())
    }
}

impl ::std::cmp::PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ::std::cmp::Ord for Task {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        if self.finished {
           if self.inner.finish_date != other.inner.finish_date {
               return self.inner.finish_date.cmp(&other.inner.finish_date).reverse();
           }
        } else if self.inner.due_date != other.inner.due_date {
            if self.inner.due_date.is_none() || other.inner.due_date.is_none() {
                return self.inner.due_date.cmp(&other.inner.due_date).reverse();
            } else {
                return self.inner.due_date.cmp(&other.inner.due_date);
            }
        }

        if self.inner.priority != other.inner.priority {
            return self.inner.priority.cmp(&other.inner.priority).reverse();
        }

        if self.inner.subject != other.inner.subject {
            return self.inner.subject.cmp(&other.inner.subject);
        }

        ::std::cmp::Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use tasks::task::*;

    #[test]
    fn markup_escape() {
        let mut task = Task::new();
        task.subject = "P&T keep focus on long term +HoWE".to_string();

        assert_eq!(
            task.markup_subject(),
            "P&amp;T keep focus on long term <b>+HoWE</b>"
        );
    }
}
