sam build --beta-features \
    --parameter-overrides DatabaseUrl=$DATABASE_URL JwtSecret=$JWT_SECRET WebClientUrl=$WEB_CLIENT_URL

sam deploy \
    --parameter-overrides DatabaseUrl=$DATABASE_URL JwtSecret=$JWT_SECRET WebClientUrl=$WEB_CLIENT_URL \
    --no-confirm-changeset \
    --no-fail-on-empty-changeset
