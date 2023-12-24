deploy:
    cargo shuttle deploy --allow-dirty

deploy_committed:
    cargo shuttle deploy

run:
    cargo shuttle run

test:
    cargo test -- --nocapture