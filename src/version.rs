use chrono::DateTime;
use duration_str::HumanFormat;
use shadow_rs::shadow;

shadow!(build);

pub fn project_name() -> &'static str {
    build::PROJECT_NAME
}
pub fn project_version() -> &'static str {
    build::PKG_VERSION
}

pub fn build_time_3339() -> &'static str {
    build::BUILD_TIME_3339
}

pub mod git {
    pub fn tag() -> Option<&'static str> {
        match super::build::TAG {
            t if t.is_empty() => None,
            t => Some(t),
        }
    }

    pub fn previous_tag() -> Option<&'static str> {
        match super::build::LAST_TAG {
            t if t.is_empty() => None,
            t => Some(t),
        }
    }

    pub fn commits_since_last_tag() -> Option<usize> {
        match (tag(), previous_tag()) {
            (Some(_), _) => Some(0),
            (_, Some(_)) => super::build::GIT_COMMITS_SINCE_TAG.parse::<usize>().ok(),
            (_, _) => None,
        }
    }

    pub fn branch() -> &'static str {
        let jenkins_branch = option_env!("GIT_BRANCH");
        match jenkins_branch {
            None | Some("") => super::build::BRANCH,
            Some(b) => b,
        }
    }

    pub fn short_commit() -> &'static str {
        super::build::SHORT_COMMIT
    }

    pub fn commit_date_3339() -> &'static str {
        super::build::COMMIT_DATE_3339
    }

    pub fn is_clean() -> bool {
        let is_jenkins = option_env!("BUILD_URL")
            .is_some_and(|url| url.starts_with("https://jenkins-deploy.vdab.be/job/"));
        is_jenkins || super::build::GIT_CLEAN
    }
}

pub mod rust {
    pub fn version() -> &'static str {
        super::build::RUST_VERSION
    }
    pub fn channel() -> &'static str {
        super::build::RUST_CHANNEL
    }
    pub fn target() -> &'static str {
        super::build::BUILD_TARGET
    }
    pub fn build_channel() -> &'static str {
        super::build::BUILD_RUST_CHANNEL
    }
}

pub fn print_version(short: bool) {
    println!("{} {}", project_name(), project_version());
    if !short {
        let build_time = DateTime::from_timestamp(build::BUILD_TIMESTAMP, 0).unwrap();
        let now = chrono::offset::Utc::now();
        let ago = now - build_time;
        let ago = ago.human_format();
        println!("build time: {} ({} ago)", build_time_3339(), ago);
        println!("git:");
        match (git::tag(), git::previous_tag()) {
            (Some(tag), _) => {
                println!("  tag: {}", tag)
            }
            (_, Some(tag)) => {
                println!(
                    "  last tag: {} ({} commits ago)",
                    tag,
                    git::commits_since_last_tag().unwrap_or_default()
                );
            }
            _ => {}
        }
        println!("  branch: {}", git::branch());
        println!("  commit: {}", git::short_commit());
        println!("  commit date: {}", git::commit_date_3339());
        println!("  clean build: {}", git::is_clean());
        println!("rust:");
        println!("  version: {}", rust::version());
        println!("  channel: {}", rust::channel());
        println!("  build channel: {}", rust::build_channel());
        println!("  target: {}", rust::target());
    }
}
