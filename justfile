deploy:
    cargo shuttle deploy --allow-dirty

deploy_committed:
    cargo shuttle deploy --idle-minutes 0

run:
    cargo shuttle run