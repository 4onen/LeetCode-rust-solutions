if [ -z "$1" ]; then
    echo "Usage: $0 <url or project_name>"
    exit 1
fi

# If the project name is a url, extract the project name
if [ "$1" != "${1##http}" ]; then
    # Extract the project name from the url
    # https://leetcode.com/problems/two-sum/ -> two_sum
    project_name=$(echo $1 | sed -Ee 's;^https?://leetcode.com/problems/([a-z0-9-]+)/?.*$;\1;' -e 's;-;_;g')
    url=$1
else
    project_name=$1
    url=$2
fi

# Remove query parameters from the url
url=${url%%\?*}

if [ -e "src/${project_name}.rs" ]; then
    echo "Project ${project_name} already exists"
    exit 1
fi

echo "Creating new project: ${project_name}.rs"
# Create the file with the leetcode template
cat > src/${project_name}.rs <<EOF
// $url

pub struct Solution;

impl Solution {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::_, _)
    }
}
EOF
echo "pub mod ${project_name};" >> src/lib.rs
rustfmt --edition 2021 <src/lib.rs >src/lib.rs.tmp
code "src/${project_name}.rs"
mv src/lib.rs.tmp src/lib.rs