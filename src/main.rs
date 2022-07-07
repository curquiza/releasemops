use semver::{Version};
use std::env;
use std::process;
use ureq::{Agent, AgentBuilder};

#[derive(Debug)]
struct Credentials {
    repository: String,
    github_token: String,
    // Slack credentials
}

fn parse_env_variables() -> Result<Credentials, String> {
    let key = "RELEASEMOPS_REPOSITORY";
    let repo = match env::var(key) {
        Ok(value) => value,
        Err(e) => return Err(format!("{}: {}", key, e))
    };

    let key = "RELEASEMOPS_GITHUB_TOKEN";
    let github_token = match env::var(key) {
        Ok(value) => value,
        Err(e) => return Err(format!("{}: {}", key, e))
    };

    let cred = Credentials {repository: repo, github_token: github_token};
    return Ok(cred);
}

fn print_error_and_exit(error_message: &str) {
    eprintln!("ERROR: {}", error_message);
    process::exit(1);
}


// rc <version>
// ------------
// - Créée branche release-v<version> si elle n’existe pas
// - Change version pour « v<version>rcX » (en fonction) directement sur la branche release-v<version>
// - Créée la pre-release sur release-v<version>
// - Envoie un message sur team-core pour rappeler comment merger les PRs (si rc0)
// - Envoie un message sur #ms-release en précisant quand sont prêts les binaires et les Docker image
fn execute_rc(cred: Credentials, version: &str) {
    // ureq ? https://github.com/algesten/ureq
    // https://lib.rs/crates/isahc
    // https://lib.rs/crates/reqwest

    let repo_url = format!("https://api.github.com/repos/{}", cred.repository.as_str());
    let body = ureq::get(repo_url.as_str())
        .set("Authorization", format!("token {}", cred.github_token.as_str()).as_str())
        .set("Accept", "application/vnd.github.v3+json")
        .call();

    match body {
        Ok(v) => println!("{:?}", v.into_string()),
        Err(e) => println!("{}", e),
    }

    // println!("body: {:?}");

    return ();
}

fn check_version(version_str: &str) -> Result<(), String>{
    if let Ok(version) = Version::parse(version_str) {
        if version.pre.is_empty() && version.build.is_empty() {
            return Ok(())
        }
    }
    return Err(format!("Invalid version {}. Valid format: vX.Y.Z", version_str))
}

fn main() {


    // if "release <version>"
    // - Change la version sur release-v<version>
    // - Merge release-v<version> dans stable
    // - Release sur stable
    // - Ramène stable dans main (ouvre une PR et set la milestone)
    // - Labelise toutes les PRs de la milestone en "done in v<version> »
    // - Message pour dire que la release est prête et quand sont dispo les binaires et les docker images

    // if "prepare-hotfix <version>"
    // - Créée branche release-v<version> si elle n’existe pas
    // - Créer Milestones si elle n’existe pas
    // - Envoie un message sur Slack (#team-core) pour dire que la branche est prête et comment merger les PRs


    // Functions
    // ---------

    // Create branche (if it does not exist)
    // Create Milestones (if it does not exist)
    // Change version: get source code, changer code, cargo build et push
    // Create PR
    // Create GitHub release/pre-release
    // Send Slack message

    // Bonus:
    // Create label (if does not exist)
    // Get Milestones issues
    // Labelize issues

    // Implementation
    // --------------

    // Get env variables
    let credentials = match parse_env_variables() {
        Ok(credentials) => credentials,
        Err(e) => return print_error_and_exit(e.as_str()),
    };

    // Read Slack message
    // ------------------
    // Temporary: read the args
    // Server qui écoute les messages slack
    // format message "releasemops <command> <version>"
    // command: rc, release, prepare-hotfix
    // version: vX.Y.Z or X.YZ, nothing more. Same behavior with v suffix or not. Donc on enlève v à <version>
    let mut args = env::args();
    let message = args.nth(1).unwrap();
    // println!("Message: {}", message);

    // Parse Slack message
    let v: Vec<&str> = message.split(' ').collect();
    if v.len() != 2 {
        print_error_and_exit("Missing information in the message");
    }
    let command = v[0].to_lowercase();
    let version = v[1].trim_start_matches("v");
    println!("Command: {}", command);
    println!("Version: {}", version);


    match check_version(version) {
        Ok(_) => println!("Valid version: {}", version),
        Err(e) => return print_error_and_exit(e.as_str()),
    };

    // Execute command
    match command.as_str() {
        "rc" => execute_rc(credentials, version),
        "release" => todo!(),
        "prepare-hotfix" => todo!(),
        _ => print_error_and_exit("Invalid command")
    }

    process::exit(0);
}
