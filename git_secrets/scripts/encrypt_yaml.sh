#!/bin/bash

scriptDir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "${scriptDir}/.." || exit 1

export SOPS_AGE_RECIPIENTS=$(cat keys/*.txt)
exec 3<<< "$(cat $1)"
sops --encrypt --input-type yaml --output-type yaml --encrypted-regex "^data$|^environment$" --age ${SOPS_AGE_RECIPIENTS} /dev/fd/3
