deploy:
    cargo shuttle deploy --allow-dirty

deploy_committed:
    cargo shuttle deploy

dev:
    cargo shuttle project start --idle-minutes 0