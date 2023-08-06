use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static CLASS_GROUP_REG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?m)((?:[!@<~\w+:_/-]|\[&?>?:?\S*\])+?)(-|:)\(((?:[~!<>\w\s:/\\,%#.$?-]|\[.*?\])+?)\)",
    )
    .unwrap()
});
static WHITESPACE_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s").unwrap());
static WHITESPACES_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
static IMPORTANCE_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(!?)(.*)").unwrap());

const SEPARATORS: [&str; 2] = ["-", ":"];
const DEPTH: u8 = 10;

fn shallow_transform(str: &str) -> String {
    let str = WHITESPACES_REG.replace_all(str, " ");

    CLASS_GROUP_REG
        .replace_all(str.trim(), |caps: &Captures| {
            if !SEPARATORS.contains(&&caps[2]) {
                return caps[0].to_string();
            }

            WHITESPACE_REG
                .split(&caps[3])
                .filter(|item| !item.is_empty())
                .map(|item| {
                    if item == "~" {
                        caps[1].to_string()
                    } else {
                        IMPORTANCE_REG
                            .replace(item, format!("${{1}}{}{}${{2}}", &caps[1], &caps[2]))
                            .to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .into_owned()
}

/// based on https://github.com/unocss/unocss/blob/main/packages/core/src/utils/variantGroup.ts
pub fn transform(str: &str) -> String {
    let mut depth = DEPTH;
    let mut previous = String::from(str);

    loop {
        let transformed = shallow_transform(&previous);
        depth -= 1;

        if transformed == previous || depth == 0 {
            break previous;
        }

        previous = transformed;
    }
}
