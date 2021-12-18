#!/bin/sh

# This script is meant to help legacy-users to migrate their
# databases so that diesel is able to use them
# Yes, I know it's a hacky solution, but then again
# it's (hopefully) only ran once on a small user(and data)base :))

# NOTE: Please backup your database before running this

rm -rf migrations/20211112185056_initial_migration
diesel migration run
git restore migrations/20211112185056_initial_migration
