# Rust release workflow using release-plz
ship version="auto":
    #!/usr/bin/env -S echo-comment --shell-flags="-euo pipefail" --color="\\033[38;5;202m"

    # 🔍 Refuse to run if not on master branch or not up to date with origin/master
    branch="$(git rev-parse --abbrev-ref HEAD)"
    if [[ "$branch" != "master" ]]; then
        # ❌ Refusing to run: not on 'master' branch (current: $branch)
        exit 1
    fi
    # 🔍 Fetch master branch
    git fetch origin master
    local_rev="$(git rev-parse HEAD)"
    remote_rev="$(git rev-parse origin/master)"
    # 🔍 Local: $local_rev\n🔍 Remote: $remote_rev
    if [[ "$local_rev" != "$remote_rev" ]]; then
        # ❌ Refusing to run: local master branch is not up to date with origin/master
        # Local HEAD:  $local_rev
        # Origin HEAD: $remote_rev
        # Please pull/rebase to update.
        exit 1
    fi

    # 🔍 Dry-run release...
    just publish --dry-run
    # ✅ Dry-run went OK, proceeding to real release

    if [[ "{{version}}" != "auto" ]]; then
        if [[ -n "$(git status --porcelain)" ]]; then
            # ❌ Working directory must be clean for manual version bump
            git status --short
            exit 1
        fi

        # 🦀 Explicit version bump with release-plz
        crate_names=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages[].name')
        for crate in $crate_names; do
          version_strings+="$crate@{{version}} "
        done

        # Trim the trailing whitespace
        version_strings=$(echo $version_strings | xargs)

        # Execute the release-plz command
        release-plz set-version $version_strings

        # 🦀 Update changelog with new version
        release-plz update --allow-dirty
    else
        # 🦀 Auto update Cargo.toml versions and changelogs
        release-plz update
    fi

    cargo check
    git add .
    just pc-fix || true
    git commit -m "chore(release): 🦀 Upgrades"

    # 🚀 Push the version bump commit
    git push --no-verify

    # 📦 Create releases and tags
    just publish


publish mode="":
    #!/usr/bin/env -S bash -euo pipefail
    git_token=$(gh auth token 2>/dev/null) || git_token=$PUBLISH_GITHUB_TOKEN

    ## 🦀 Let release-plz handle workspace crate tagging
    ## It will create tags like: plaudit-cli-v0.1.0, plaudit-core-v0.1.0, etc.
    release-plz release --backend github --git-token $git_token {{mode}}


# For when release-plz isn't working
ship-manual:
    #!/usr/bin/env -S echo-comment --shell-flags="-euo pipefail" --color="\\033[38;5;202m"

    # Bump patch version
    cargo set-version --bump patch

    # Commit and tag
    git add .
    git commit -m "chore(release): 🦀 Upgrades"

    # Tag each crate
    crate_names=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages[].name')
    for crate in $crate_names; do
      version=$(cargo metadata --no-deps --format-version 1 | jq -r ".packages[] | select(.name==\"$crate\") | .version")
      git tag "${crate}-v${version}"
    done

    # Push to remote
    git push
    git push --tags

    # Publish core first (cli depends on it)
    cargo publish -p plaudit-core
    sleep 30
    cargo publish -p plaudit-cli
