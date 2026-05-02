[private]
list:
    @just -l

dev:
    dx serve

build:
    dx bundle --web --ssg --release

clean:
    rm -rf target

assert-git-clean:
    #!/bin/bash
    if [[ -n "$(git status --porcelain 2>/dev/null)" ]]; then
        echo "Git working tree is dirty. Please commit or stash your changes."
        exit 1
    fi

deploy: assert-git-clean build
    npx wrangler pages deploy \
        target/dx/samutils/release/web/public \
        --project-name samutils