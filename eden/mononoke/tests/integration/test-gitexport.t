# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This software may be used and distributed according to the terms of the
# GNU General Public License found in the LICENSE file in the root
# directory of this source tree.

Setting up a simple scenario for the gitexport tool
  $ . "${TEST_FIXTURES}/library.sh"


Setup configuration
  $ REPOTYPE="blob_files"
  $ setup_common_config "$REPOTYPE"
  $ cd $TESTTMP


Set some env vars that will be used frequently

  $ EXPORT_DIR="export_dir"
  $ EXPORT_SUBDIR="$EXPORT_DIR/subdir_to_export"
-- Folder that should NOT be exported to the git repo
  $ INTERNAL_DIR="internal_dir"
  $ SECOND_EXPORT_DIR="second_export_dir"


# -------------------------- Create commits --------------------------
  $ testtool_drawdag -R repo --derive-all --no-default-files <<EOF
  > A-B-C-D-E-F-G-H-I-J-K
  > # modify: A "$EXPORT_DIR/B.txt" "File to export"
  > # message: A "Add files to export dir"
  > # modify: B "$INTERNAL_DIR/internal.txt" "Internal file"
  > # message: B "Add file to internal_dir"
  > # modify: C "$EXPORT_SUBDIR/export_file_in_subdir.txt" "File in export subdirectory"
  > # message: C "Add subdirectory to export dir"
  > # modify: D "$EXPORT_SUBDIR/second_subdir_export.txt" "File in export subdirectory"
  > # modify: D "$EXPORT_DIR/C.txt" "File to export"
  > # modify: D "$INTERNAL_DIR/another_internal.txt" "Internal file"
  > # message: D "Add files to all directories"
  > # modify: E "$SECOND_EXPORT_DIR/another_file.txt" "Another file to export"
  > # message: E "Create another export directory"
  > # modify: F "$INTERNAL_DIR/internal.txt" "Changing file"
  > # modify: F "$EXPORT_DIR/A.txt" "Changing file"
  > # modify: F "$EXPORT_SUBDIR/exception_from_export_dir.txt" "Changing file"
  > # message: F "Modify internal and exported files"
  > # modify: G "$EXPORT_DIR/B.txt" "Changing file"
  > # message: G "Modify only exported file"
  > # modify: H "$EXPORT_SUBDIR/second_subdir_export.txt" "Changing file"
  > # message: H "Modify only file in export subdirectory"
  > # modify: I "$INTERNAL_DIR/another_internal.txt" "Changing file"
  > # message: I "Modify only file in internal root"
  > # delete: J "$EXPORT_SUBDIR/second_subdir_export.txt"
  > # delete: J "$INTERNAL_DIR/another_internal.txt"
  > # message: J "Delete internal and exported files"
  > # modify: K "root_file.txt" "Root file"
  > # message: K "Add file to repo root"
  > # bookmark: K master
  > EOF
  A=2b45b0cac2615a6b5f1808161f96eb56376f313b45744ce83fd60931dee1e02b
  B=db859048f5ffc6d47dddd3bbe01e223654e9992537421e4ba13b87a7e0dbcc3c
  C=18ecf80ae5c1d7f1ca4d86f0679553c96be5aff1fb7b6dfa7b6343c0cde461a5
  D=b1075aab50713f6440222a3e8729d874fab9e3276fd97057ebda2bea4fc27e68
  E=bf427657abaa0a5b88cf50295ba5c5639f45b89cc67e15f7bc5c2b496c84bff9
  F=22bf902c5e155b92caddfe384693a69f379cdada5277ab524a8dbfddc5ab2077
  G=ae2469ceeba5ee03e6501c85b7335c1fa5fa8e75a5de678743037d6e8c220c47
  H=aad9a55aa109275b392b829d09c571caa4add25753c6a6d547d753534e8ddc89
  I=83f4af124d0b2052d090ca254150f6fa4d5dc9303ffd23c601d1f7a6dc23892e
  J=56abf334447e5deb10163335caf2477aa105a8bee096627de06222f01d45c65d
  K=ca1b7e33632b3b9a89abe7f820b590f1185cf7e187386e9bddf4c1cbe62dc324

  $ start_and_wait_for_mononoke_server
  $ hgmn_clone mononoke://$(mononoke_address)/repo repo
  $ cd repo
  $ hg -q co master


# Finish creating commits
# ------------------------------------------------------------------------------


Check all the commits
  $ hg log --stat
  commit:      52505ac48534
  bookmark:    master
  bookmark:    default/master
  hoistedname: master
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Add file to repo root
  
   root_file.txt |  1 +
   1 files changed, 1 insertions(+), 0 deletions(-)
  
  commit:      3a325085e7ee
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Delete internal and exported files
  
   export_dir/subdir_to_export/second_subdir_export.txt |  1 -
   internal_dir/another_internal.txt                    |  1 -
   2 files changed, 0 insertions(+), 2 deletions(-)
  
  commit:      01d2fb7ebd41
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Modify only file in internal root
  
   internal_dir/another_internal.txt |  2 +-
   1 files changed, 1 insertions(+), 1 deletions(-)
  
  commit:      103d89f67b17
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Modify only file in export subdirectory
  
   export_dir/subdir_to_export/second_subdir_export.txt |  2 +-
   1 files changed, 1 insertions(+), 1 deletions(-)
  
  commit:      ca615231c1ca
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Modify only exported file
  
   export_dir/B.txt |  2 +-
   1 files changed, 1 insertions(+), 1 deletions(-)
  
  commit:      ec9cb569042a
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Modify internal and exported files
  
   export_dir/A.txt                                          |  1 +
   export_dir/subdir_to_export/exception_from_export_dir.txt |  1 +
   internal_dir/internal.txt                                 |  2 +-
   3 files changed, 3 insertions(+), 1 deletions(-)
  
  commit:      6c58592d63a0
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Create another export directory
  
   second_export_dir/another_file.txt |  1 +
   1 files changed, 1 insertions(+), 0 deletions(-)
  
  commit:      a47d0a3f0aa3
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Add files to all directories
  
   export_dir/C.txt                                     |  1 +
   export_dir/subdir_to_export/second_subdir_export.txt |  1 +
   internal_dir/another_internal.txt                    |  1 +
   3 files changed, 3 insertions(+), 0 deletions(-)
  
  commit:      e5f8b4c6481b
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Add subdirectory to export dir
  
   export_dir/subdir_to_export/export_file_in_subdir.txt |  1 +
   1 files changed, 1 insertions(+), 0 deletions(-)
  
  commit:      c7748d4b5c89
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Add file to internal_dir
  
   internal_dir/internal.txt |  1 +
   1 files changed, 1 insertions(+), 0 deletions(-)
  
  commit:      d5d07c2e7885
  user:        author
  date:        Thu Jan 01 00:00:00 1970 +0000
  summary:     Add files to export dir
  
   export_dir/B.txt |  1 +
   1 files changed, 1 insertions(+), 0 deletions(-)
  


# -------------------- Use the gitexport tool --------------------

Set location of binary, resources and options (e.g. output path, directories)
# TODO(T160600991): Pass the CLI pass once the initial binary is setup in buck
  $ GITEXPORT_CLI=""

# Path that should be exported to the git repo
  $ EXPORT_PATHS="$EXPORT_DIR $SECOND_EXPORT_DIR"

  $ HG_REPO="$TESTTMP/repo"

  $ GIT_REPO_OUTPUT="$TESTTMP/git_repo"

# TODO(T160600443): support optional first/last commits
# NOTE: these would take precedence over the start/end date arguments.
  $ FIRST_COMMIT=""

  $ LAST_COMMIT=""

# TODO(T160600443): support optional start/end date arguments
  $ START_DATE="2023-01-01"

  $ END_DATE="2023-02-01"

Run the tool

# TODO(T160600991): uncomment once the CLI binary is created
# $ $GITEXPORT_CLI --hg-repo "$REPO" --output "$GIT_REPO_OUTPUT" --export-paths "$EXPORT_PATHS"



# -------------------- Run checks on the git repo --------------------


# $ cd "$GIT_REPO_OUTPUT"


# TODO(T160600934): count number of commits
# TODO(T160600934): assert paths are correct
# TODO(T160600934): confirm no internal files are there
