#!/usr/bin/env -S just --justfile

default:
  @just --list

import '.just/build.just'
import '.just/db.just'
import '.just/release.just'
