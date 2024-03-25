# !/bin/bash

git --version
git config --global pager.branch false
git branch -al main
git branch -al release-1
git branch -al 'release-branch-*'