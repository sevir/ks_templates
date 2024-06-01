# Automatic encrypt/decrypt of secrets files using SOPS and AGE

## Requirements

- You need to be sure that you have the following tools installed:
  - [SOPS](https://github.com/getsops/sops/releases)
  - [AGE](https://github.com/FiloSottile/age/releases)

## Generate an AGE key pair

```bash
mkdir -p  ~/.config/sops/age/
age-keygen -o ~/.config/sops/age/keys.txt
```

The `keys.txt` file will contain the private key, and the public key, so keep it safe and saves it in a backup location. The command prints the public key to the terminal, so you can copy it in a file in the keys folder.

## Commit and request a new recrypt of the secrets files

Commit your public key in the repo and request for a partner to recrypt the secrets files with your public key.

## Install the git filters

```bash
./scripts/install_git_filters.sh
```

## Pull the repo and check

```bash
git pull
```

If the secrets files are not decrypted, then remove the file and resets de repo, the files will be decrypted on the fly.

## Enabling new secrets files

Add the following line to the `.gitattributes` file:

```
docker-compose.yml filter=sops_type
# where type is the file type, e.g. yaml, json or dotenv.
```
