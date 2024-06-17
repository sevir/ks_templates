#!/bin/bash

scriptDir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "${scriptDir}/.." || exit 1

export SOPS_AGE_RECIPIENTS=$(find . -name 'keys/*.txt' -print0 | xargs -0 sed 's/$/,/' | tr -d '\n')
exec 3<<< "$(cat $1)"
sops --encrypt --input-type dotenv --output-type dotenv --age ${SOPS_AGE_RECIPIENTS} /dev/fd/3
