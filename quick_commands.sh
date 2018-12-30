#!/bin/bash

# Create new cargo project quickly
function eulercargo() {
  # Read arguments and convert them to lower case
  LOWER_CASE_ARGS=$(echo "$*" | tr '[:upper:]' '[:lower:]')

  # Replace all spaces with underscores
  FOLDER_NAME=${LOWER_CASE_ARGS//' '/'_'}
  # Remove leading 000_
  PROJECT_NAME=${FOLDER_NAME/[0-9][0-9][0-9]_/''}

  # Created a cargo folder with 000_problem_name name and name the project problem_name
  # Do not use VCS because everything is already stored in a parent git folder
  cargo new $FOLDER_NAME --name $PROJECT_NAME --vcs none

  # Reading all projects folder names and saving them as members of Cargo.toml's workspace
  TOML="[workspace]\n\nmembers = [\n"
  for folder in $(ls --ignore="*.*" --ignore target); do
    TOML="$TOML  \"$folder\",\n"
  done
  TOML="$TOML]"
  echo $TOML > Cargo.toml

  # Explaining Cargo warning:
  echo -e "\n\e[96mInfo:\e[0m a warning might appear about $PROJECT_NAME being missing from Cargo.toml."
  echo "You can safely dismiss that as I just added it.\n"

  # Output the created names for confirmation
  echo "Created $PROJECT_NAME in $FOLDER_NAME. Goodluck!"
}
