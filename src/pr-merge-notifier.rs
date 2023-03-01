use anyhow::{Error, Result};
use github_flows::{get_octo, listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event("jaykchen", "vitesse-lite", vec!["pull_request"], handler).await;

    Ok(())
}

async fn get_email(user: &str) -> anyhow::Result<()> {
    let octo = get_octo(None);
    let query_str = format!("/users/{user}");

    let response = octo.get(query_str, None::<&()>).await?;
    let text = format!("{:?}", response);
    send_message_to_channel("ik8", "general", "supposedly done query".to_string());
    send_message_to_channel("ik8", "general", text);
    Ok(())
    // match response.get("email") {
    //     Some(e) => Some(e.to_string()),
    //     None => None,
    // }
}

async fn handler(payload: EventPayload) {
    let mut title: String = "no title found".to_string();
    let mut merged: bool = false;
    let mut html_url: String = "no html_url found".to_string();
    let mut user: String = "".to_string();

    if let EventPayload::PullRequestEvent(e) = payload {
        let pr = e.pull_request;
        title = pr.title.expect("no title");
        html_url = pr.html_url.expect("no html_url field").to_string();
        user = pr.user.expect("user not found").login;
        merged = match pr.merge_commit_sha {
            Some(_sha) => true,
            None => false,
        };
    }

    get_email(&user).await;

    // match get_email(&user).await {
    //     None => {}
    //     Some(email) => {
    // let merged_str = if merged { "merged" } else { "null" };
    // let text = format!("title: {title}\n html_url: {html_url}\n user: {user}\n, email: email\n merged: {merged_str}");
    // send_message_to_channel("ik8", "general", text);
    // }
    // }
}
