# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        -*)
            # Handle optional parameters
            case "$1" in
                -v|--verbose)
                    shift
                    echo "verbose mode enabled"
                    verbose=true
                    ;;
                *)
                    echo "Unknown option: $1"
                    exit 1
                    ;;
            esac
            ;;
        *)
            # Handle positional parameters (assumed to be required)
            if [ -z "$required_param" ]; then
                required_param="$1"
            else
                echo "Unexpected positional argument: $1"
                exit 1
            fi
            ;;
    esac
    shift
done

# Check if the required parameter is provided
if [ -z "$required_param" ]; then
    echo "Required parameter not provided."
    exit 1
fi




find . -name "*.code-workspace" -print0 | xargs -r -0 ls -1  -t | head -1
echo "------------------------"
workspace_file=$(find . -name "*.code-workspace" -print0 | xargs -r -0 ls -1 -t | head -1)
echo "Workspace file found: $workspace_file"

test1="day1/Cargo.toml" 
if ! grep -q "$test1" "$workspace_file"; then 
    echo "Something's gone wrong here"
else 
    echo "All good still"
fi

workspace= find *.code-workspace
echo $workspace