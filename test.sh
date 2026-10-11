set -euo pipefail

for manifest in */Cargo.toml; do
    echo "Testing ${manifest%/Cargo.toml}"
    cargo test --manifest-path "$manifest"
done
