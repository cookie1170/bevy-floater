[confirm]
[arg('ver', pattern='(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?')]
release ver:
    @if test -n "$(git status --porcelain)"; then \
        echo "{{ style('error') }}error: {{ NORMAL }}{{ style('command') }}Git working directory must be clean!{{ NORMAL }}"; \
        exit 1; \
    fi

    @echo "Releasing version {{ver}}"
    sed -ir 's/version = ".*"/version = "{{ver}}"/' Cargo.toml
    git add Cargo.toml
    git commit -m "v{{ver}}"
    git tag -a v{{ver}}
    git push --follow-tags
    gh release create v{{ver}} --notes-from-tag
    cargo publish
