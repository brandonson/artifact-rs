#!/bin/bash

if [ "$TRAVIS_REPO_SLUG" == "brandonson/artifact-rs" ] && [ "$TRAVIS_PULL_REQUEST" == "false"] && [ "$TRAVIS_BRANCH" == "master" ] then


  echo "Pushing rustdocs to github pages."

  cargo doc --no-deps
  echo '<meta http-equiv=refresh content=0;url=artifact/index.html>' > target/doc/index.html
  sudo pip install ghp-import
  ghp-import -n target/doc
  git push -qf https://${TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages

  echo "Rustdoc documentation updated."
else
  echo "Not pushing docs to github pages."
fi
