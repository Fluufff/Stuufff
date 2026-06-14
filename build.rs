use std::io::Write;
use std::{fs::File, process::Command};

use shadow_rs::{SdResult, ShadowBuilder};

fn main() {
    println!("cargo::rerun-if-changed=.git/HEAD");
    ShadowBuilder::builder()
        .hook(commits_since_tag)
        .build()
        .unwrap();
}

fn commits_since_tag(mut file: &File) -> SdResult<()> {
    let tag = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .output()
        .map(|x| {
            String::from_utf8(x.stdout)
                .map(|x| x.trim().to_string())
                .ok()
        })
        .unwrap()
        .unwrap();

    let commit_count = Command::new("git")
        .args(["rev-list", &format!("{}..HEAD", &tag), "--count"])
        .output()
        .map(|x| {
            String::from_utf8(x.stdout)
                .map(|x| x.trim().to_string())
                .ok()
        })
        .unwrap()
        .unwrap();

    writeln!(
        file,
        r#"
        pub const GIT_COMMITS_SINCE_TAG: &str = "{}";
        "#,
        commit_count
    )?;
    Ok(())
}
