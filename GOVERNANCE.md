# Governance

This document explains who makes decisions for the Soroban Security Scanner,
how proposals are accepted, and how disagreements are resolved. It is
deliberately lightweight: the project is small, and the goal is to make the
process predictable rather than to add ceremony.

## Roles

- **Maintainer** — merges changes, manages releases, protects `main`, and makes
  the final call when consensus cannot be reached. The current maintainer is
  listed in the [README](README.md#how-this-project-is-run).
- **Contributor** — anyone who opens an issue, proposes an ADR, or submits a
  pull request. Contributors do not need prior approval to propose work, but
  **assignment is required before starting** work that is part of a wave.
- **Reviewer** — a contributor or maintainer who reviews a pull request. Reviews
  check correctness, tests, security impact, and documentation, not style
  preferences.
- **Security reporter** — anyone reporting a vulnerability. Security reports are
  handled privately under [SECURITY.md](SECURITY.md); they are not governed by
  the public proposal process.

## Decision making

1. **Routine changes** (bug fixes, documentation, tests, small features) are
   decided by pull-request review. CI must be green and at least one maintainer
   must approve before merge.
2. **Scoped work** should have an issue with a problem statement, acceptance
   criteria, tests, and a complexity label. If the scope is unclear, discuss it
   in the issue before writing code.
3. **Significant or hard-to-reverse changes** (public CLI/JSON/SARIF contracts,
   dependency policy, security model, release process) require an
   [Architecture Decision Record](docs/architecture/adr/README.md) before
   implementation. The ADR is reviewed in the same pull request as the change.
4. **Contentious changes** start as an issue or draft ADR. The proposer
   summarises the options and trade-offs, and the maintainer allows at least
   seven days for comment unless the change is urgent (for example, an active
   security issue). The maintainer then records the decision and its rationale
   in the issue or ADR. Silence is not approval; a decision is always stated.
5. **Reversals** follow the same path as the original decision. An accepted ADR
   is superseded by a new ADR rather than silently rewritten.

Lazy consensus applies to small, reversible changes: if no concern is raised
during review and the checks pass, the change may be merged. It does not apply
to security-sensitive code, dependency additions, or public contract changes.

## ADR process

- ADRs live in [`docs/architecture/adr/`](docs/architecture/adr/README.md) and
  use the next free four-digit number.
- Each ADR states its context, decision, alternatives considered, and
  consequences. Status is one of `Proposed`, `Accepted`, `Superseded`, or
  `Rejected`.
- Add the ADR to the index in the same pull request. A superseded ADR keeps its
  file and links to its replacement.
- Dependency additions must also satisfy
  [ADR-0005](docs/architecture/adr/0005-dependency-justification-policy.md).

## Pull requests and releases

- `main` is protected by review and CI. Contributors work on branches and open a
  pull request that links its issue with `Closes #<issue>`.
- A pull request is mergeable when it is focused, tested, documented where
  user-visible, and green on the local gate in
  [CONTRIBUTING.md](CONTRIBUTING.md).
- Maintainers merge pull requests and do not merge their own unreviewed
  security-sensitive changes.
- Releases follow [the release process](docs/development/release.md). Versions
  follow semantic versioning, and user-visible changes are recorded in
  [`CHANGELOG.md`](CHANGELOG.md).
- Release tags and published artifacts are created by the maintainer; a failed
  release is fixed forward with a new version, never by rewriting a published
  tag.

## Changing this document

Governance changes use the same pull-request flow as code. Because they affect
every contributor, they need maintainer approval and a short rationale in the
pull request. Trivial clarifications may be merged under normal review.

## Code of conduct and security

Participation is governed by [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Report
vulnerabilities privately as described in [SECURITY.md](SECURITY.md), not in a
public issue or pull request.
