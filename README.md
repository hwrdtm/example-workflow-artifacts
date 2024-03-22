# What

This is an example repo to demonstrate how to upload and download artifacts across separate workflows.

Pushing to `release-1` branch will result in building the Rust crate and uploading it to Github.

Pushing to any PR branch will result in downloading the latest build from `release-1` and then running it in that CI workflow run.