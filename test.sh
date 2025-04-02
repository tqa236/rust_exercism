for path in */; do
    [ -d "${path}" ] || continue # if not a directory, skip
    [[ "$(basename "${path}")" == .* ]] && continue
    dirname="$(basename "${path}")"
    cd "$dirname" || exit
    cargo test
    if [ $? -ne 0 ]; then
        echo "Tests failed in $dirname"
        exit 1
    fi
    cd .. || exit
done