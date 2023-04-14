#!/usr/bin/env bash

# retrieves broc_releases.json, expects AUTH_HEADER to be set

# https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/
set -euxo pipefail

curl --request GET \
          --url https://api.github.com/repos/roc-lang/broc/releases \
          --header '$AUTH_HEADER' \
          --header 'content-type: application/json' \
          --output broc_releases.json