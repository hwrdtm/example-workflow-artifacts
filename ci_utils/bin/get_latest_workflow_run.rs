use ci_utils::get_target_branch;
use log::{debug, info};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Get the name of the workflow from the 1st command line argument.
    let workflow_name = std::env::args().nth(1).expect("No workflow name provided");
    // Get the name of the target branch from the 2nd command line argument.
    let target_branch = std::env::args().nth(2).expect("No target branch provided");
    let target_branch = get_target_branch(&target_branch).expect("Failed to get target branch");

    // Remove the 'origin/' prefix if it exists.
    let target_branch = target_branch.trim_start_matches("origin/");

    // Fetch the workflow from the REST API.
    let url = "https://api.github.com/repos/hwrdtm/example-workflow-artifacts/actions/workflows";
    debug!("Fetching workflows from: {}", url);
    let req = populate_github_api_headers(reqwest::Client::new().get(url))
        .send()
        .await
        .expect("Failed to fetch Github workflows");
    // Deserialize the response into a workflow list.
    let workflow_list: WorkflowList = req
        .json()
        .await
        .expect("Failed to deserialize workflow list");
    info!("Workflow list: {:?}", workflow_list);

    // Find the workflow with the given name.
    let workflow = workflow_list
        .workflows
        .iter()
        .find(|w| w.name == workflow_name)
        .expect("No workflow found with the given name");

    // Fetch the workflow runs from the REST API.
    let url = format!(
        "https://api.github.com/repos/hwrdtm/example-workflow-artifacts/actions/workflows/{}/runs?branch={}",
        workflow.id, target_branch
    );
    debug!("Fetching workflow runs from: {}", url);
    let req = populate_github_api_headers(reqwest::Client::new().get(url))
        .send()
        .await
        .expect("Failed to fetch Github workflow runs");
    // Deserialize the response into a workflow run list.
    let workflow_run_list: WorkflowRunList = req
        .json()
        .await
        .expect("Failed to deserialize workflow run list");
    info!("Workflow run list: {:?}", workflow_run_list);

    // Get the first workflow run from the list.
    let workflow_run = workflow_run_list
        .workflow_runs
        .first()
        .expect("No workflow run found");

    // Print the ID of the first workflow run.
    println!("{}", workflow_run.id);
}

fn populate_github_api_headers(req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    req.header("Accept", "application/vnd.github.v4+json")
        .header(
            "Authorization",
            format!("Bearer {}", std::env::var("GH_PAT").unwrap()),
        )
        .header("X-Github-Api-Version", "2022-11-28")
        .header("User-Agent", "hwrdtm-downloader")
}

#[derive(Debug, Deserialize)]
struct WorkflowList {
    total_count: u64,
    workflows: Vec<Workflow>,
}

#[derive(Debug, Deserialize)]
struct Workflow {
    id: u64,
    node_id: String,
    name: String,
    path: String,
    state: String,
    created_at: String,
    updated_at: String,
    url: String,
    html_url: String,
    badge_url: String,
}

#[derive(Debug, Deserialize)]
struct WorkflowRunList {
    total_count: u64,
    workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
struct WorkflowRun {
    id: u64,
    node_id: String,
    head_branch: String,
    head_sha: String,
    run_number: u64,
    event: String,
    status: String,
    conclusion: String,
    workflow_id: u64,
    url: String,
    html_url: String,
    pull_requests: Vec<String>,
    created_at: String,
    updated_at: String,
    jobs_url: String,
    logs_url: String,
    check_suite_url: String,
    artifacts_url: String,
    cancel_url: String,
    rerun_url: String,
    workflow_url: String,
}
