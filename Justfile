[private]
list:
    @just -l

deploy:
    dx bundle --web --ssg --release
    npx wrangler pages deploy \
        target/dx/samutils/release/web/public \
        --project-name samutils