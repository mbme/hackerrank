#!/usr/bin/env bash
set -e # fail on error

if [ $# -eq 0 ]; then
    echo "contest url must be provided"
    exit 1
fi

URL="$1"

# https://stackoverflow.com/a/8979741
CONTEST_NAME="${URL##*/}"

echo "preparing for the contest $CONTEST_NAME"

SRC_DIR="$CONTEST_NAME/src"
TMP_DIR="$CONTEST_NAME/tmp"

mkdir -v "$CONTEST_NAME"
mkdir -v "$CONTEST_NAME/src"
mkdir -v "$CONTEST_NAME/tmp"

# download problem statement
STATEMENT_URL="https://www.hackerrank.com/rest/contests/master/challenges/$CONTEST_NAME/download_pdf?language=English"
STATEMENT_FILE_NAME="statement.pdf"
wget "$STATEMENT_URL" -O "$TMP_DIR/$STATEMENT_FILE_NAME"

# download archive with test cases
CONTEST_TEST_CASE_URL="https://www.hackerrank.com/rest/contests/master/challenges/$CONTEST_NAME/download_testcases"
TMP_ZIP="$TMP_DIR/tmp.zip"
wget "$CONTEST_TEST_CASE_URL" -O "$TMP_ZIP"
# unarchive it in the temp dir
unzip "$TMP_ZIP" -d "$TMP_DIR"
# and remove archive
rm "$TMP_ZIP"

# copy required files
cp "_tmp/main.rs" "$SRC_DIR"
ln "_tmp/test.sh" "$CONTEST_NAME/"

# init README
cat >"$CONTEST_NAME/README.md" <<EOF
# Hackerrank contest [$URL]($URL)

Problem statement [tmp/$STATEMENT_FILE_NAME](tmp/$STATEMENT_FILE_NAME).

EOF

echo "finished preparing to the contest $CONTEST_NAME"
