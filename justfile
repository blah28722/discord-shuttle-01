deploy:
    cargo shuttle deploy

deploy_dirty:
    cargo shuttle deploy --allow-dirty

run:
    cargo shuttle run

test:
    cargo test -- --nocapture