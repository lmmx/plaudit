bin := "plaudit-cli"

import ".just/cargo.just"
import ".just/commit.just"
import ".just/hooks.just"
import ".just/plaudit.just"
import ".just/release.just"

default:
    @just --list
