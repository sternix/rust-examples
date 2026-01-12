#!/bin/sh

find ../ -type f -name '*.rs' -exec rustfmt --edition 2024 {} ';'