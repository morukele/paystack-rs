crates=(
    macros
    models
    http
)

for crate in "${crates[@]}"; do
    echo "Publishing ${crate}"
    (
      cd "$crate"
      cargo publish --no-verify
    )
    sleep 20
done

cargo publish