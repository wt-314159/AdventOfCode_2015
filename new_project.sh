#!/bin/bash

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        -*)
            # Handle optional parameters
            case "$1" in
                # Enable verbose mode
                -v|--verbose)
                    shift
                    echo "Verbose mode enabled."
                    verbose=true
                    ;;
                # Can add other optional parameters to handle here
                *)
                    echo "Unknown option: $1"
                    exit 1
                    ;;
            esac
            ;;
        *)
            # Handle positional parameters (assumed to be required)
            if [ -z "$project_name" ]; then
                project_name="$1"
            else
                echo "Unexpected positional argument: $1"
                exit 1
            fi
            ;;
    esac
    shift
done

# Check if the required parameter is provided
if [ -z "$project_name" ]; then
    echo "Required parameter not provided."
    exit 1
fi

# make directory with project name
mkdir "$project_name"

# copy template files into new directory
if [ "$verbose" = true ]; then
    echo "Copying template to $project_name directory"
fi
cp -r template/* "$project_name"

# Find the workspace file (newest file matching *.code-workspace)
workspace_file=$(find . -name "*.code-workspace" -print0 | xargs -r -0 ls -1 -t | head -1)
# Project path 
# ( Full path would be "$(pwd)/%project_name/Cargo.toml" )
project_path="$project_name/Cargo.toml"

# if verbose mode enabled, echo what we're doing
if [ "$verbose" = true ]; then
    echo "Project path: $project_path"
    echo "Updating $workspace_file..."
fi

if ! grep -q "$project_path" "$workspace_file"; then
    # Add the project path to the linkedProjects array
    jq ".settings.\"rust-analyzer.linkedProjects\" += [\"$project_path\"]" "$workspace_file" > tmpfile && mv tmpfile "$workspace_file"
fi