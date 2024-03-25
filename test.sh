# !/bin/bash

git --version
git config --global pager.branch false

# Run git branch -al main and trim the first character
git branch -al main | cut -c 3-
git branch -al release-1
git branch -al 'release-branch-*'