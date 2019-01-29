include "common/fb303/if/fb303.thrift"

namespace py scm.mononoke.apiserver.thrift.apiserver
namespace py3 scm.mononoke.apiserver.thrift

enum MononokeAPIExceptionKind {
  InvalidInput = 1,
  NotFound = 2,
  InternalError = 3,
}

exception MononokeAPIException {
  1: MononokeAPIExceptionKind kind,
  2: string reason,
}

struct MononokeGetRawParams {
  1: string repo,
  2: string changeset,
  3: binary path,
}

struct MononokeGetChangesetParams {
    1: string repo,
    3: string revision,
}

struct MononokeChangeset {
  1: string commit_hash,
  2: string message,
  3: i64 date,
  4: string author,
  5: list<string> parents
  6: map<string, binary> extra,
}

struct MononokeBranches {
  1: map<string, string> branches,
}

struct MononokeGetBranchesParams{
  1: string repo,
}

service MononokeAPIService extends fb303.FacebookService {
  binary get_raw(1: MononokeGetRawParams params)
    throws (1: MononokeAPIException e),

  MononokeChangeset get_changeset(1: MononokeGetChangesetParams param)
    throws (1: MononokeAPIException e),

  MononokeBranches get_branches(1: MononokeGetBranchesParams params)
    throws (1: MononokeAPIException e),
}
