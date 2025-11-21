const MAGIC_COMMENT_DIFF_ADD: &str = "// diff-add";
const MAGIC_COMMENT_DIFF_ADD_START: &str = "// diff-add-start";
const MAGIC_COMMENT_DIFF_ADD_END: &str = "// diff-add-end";
const MAGIC_COMMENT_DIFF_DEL: &str = "// diff-del";
const MAGIC_COMMENT_DIFF_DEL_START: &str = "// diff-del-start";
const MAGIC_COMMENT_DIFF_DEL_END: &str = "// diff-del-end";

#[derive(PartialEq, Eq)]
enum Diff {
    /// Line was not changed.
    None,
    /// Line was added.
    Add,
    /// Line was deleted.
    Del,
}

impl Diff {
    /// Returns a [Diff] basing on the first character in the string.
    fn new(s: &str) -> Self {
        if s.starts_with("+") {
            Self::Add
        } else if s.starts_with("-") {
            Self::Del
        } else {
            Self::None
        }
    }
}

struct Group {
    diff: Diff,
    lines: Vec<String>,
}

impl Group {
    fn new(diff: Diff, line: String) -> Self {
        Self {
            diff,
            lines: vec![line],
        }
    }

    fn add(&mut self, line: String) {
        self.lines.push(line);
    }
}

pub fn apply(mut lines: Vec<String>) -> Vec<String> {
    let mut groups: Vec<Group> = vec![];
    for line in lines.drain(..) {
        let diff = Diff::new(&line);
        if let Some(group) = groups.last_mut() {
            if group.diff == diff {
                group.add(line);
            } else {
                groups.push(Group::new(diff, line));
            }
        } else {
            groups.push(Group::new(diff, line));
        }
    }
    let mut output = vec![];
    for mut group in groups {
        match group.diff {
            Diff::None => {
                output.append(&mut group.lines);
            }
            Diff::Add => {
                let count = group.lines.len();
                if count > 1 {
                    output.push(MAGIC_COMMENT_DIFF_ADD_START.to_string());
                } else {
                    output.push(MAGIC_COMMENT_DIFF_ADD.to_string());
                }
                output.append(&mut group.lines);
                if count > 1 {
                    output.push(MAGIC_COMMENT_DIFF_ADD_END.to_string());
                }
            }
            Diff::Del => {
                let count = group.lines.len();
                if count > 1 {
                    output.push(MAGIC_COMMENT_DIFF_DEL_START.to_string());
                } else {
                    output.push(MAGIC_COMMENT_DIFF_DEL.to_string());
                }
                output.append(&mut group.lines);
                if count > 1 {
                    output.push(MAGIC_COMMENT_DIFF_DEL_END.to_string());
                }
            }
        }
    }
    output
}
