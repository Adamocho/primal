#! /usr/bin/env bash

DIRNAME=$(dirname "$0")
FIRST="${HOME}/.config/nvim/syntax"
SECOND="${HOME}/.vim/syntax"

set -x;

mkdir -p "${FIRST}"
mkdir -p "${SECOND}"

cp --verbose --interactive "${DIRNAME}/pml.vim" "${FIRST}"
cp --verbose --interactive "${DIRNAME}/pml.vim" "${SECOND}"
