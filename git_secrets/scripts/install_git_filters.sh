#!/bin/bash

git config --local filter.sops_yaml.smudge $(pwd)/scripts/decrypt_yaml.sh
git config --local filter.sops_yaml.clean $(pwd)/scripts/encrypt_yaml.sh
git config --local filter.sops_yaml.required true

git config --local filter.sops_json.smudge $(pwd)/scripts/decrypt_json.sh
git config --local filter.sops_json.clean $(pwd)/scripts/encrypt_json.sh
git config --local filter.sops_json.required true

git config --local filter.sops_env.smudge $(pwd)/scripts/decrypt_env.sh
git config --local filter.sops_env.clean $(pwd)/scripts/encrypt_env.sh
git config --local filter.sops_env.required true