  $ hg init outer
  $ cd outer

hg debugsub with no remapping

  $ echo 'sub = http://example.net/libfoo' > .hgsub
  $ hg add .hgsub

  $ hg debugsub
  path sub
   source   http://example.net/libfoo
   revision 

hg debugsub with remapping

  $ cat > .hg/hgrc <<EOF
  > [subpaths]
  > http://example.net = ssh://localhost
  > EOF

  $ hg debugsub
  path sub
   source   ssh://localhost/libfoo
   revision 

test bad subpaths pattern

  $ cat > .hg/hgrc <<EOF
  > [subpaths]
  > .* = \1
  > EOF
  $ hg debugsub
  abort: bad subrepository pattern in .*/test-subrepo-paths.t/outer/.hg/hgrc:2: invalid group reference

  $ exit 0
