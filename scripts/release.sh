#!/bin/sh
# shellcheck shell=dash

cargo release \
    --skip-publish \
    patch
git push \
    --push-option merge_request.create \
    --push-option merge_request.target=master \
    --push-option merge_request.merge_when_pipeline_succeeds \
    --push-option merge_request.title="New release" \
    --follow-tags
