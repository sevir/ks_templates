#!/bin/bash

scriptDir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "${scriptDir}/.." || exit 1

exec 3<<< "$(cat $1)"
sops --decrypt --input-type json --output-type json /dev/fd/3
