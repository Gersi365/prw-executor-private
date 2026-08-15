# Phase 039 — Admission Pipeline Visibility Hardening

## Objective

Ensure the policy-admission and successful-response pipeline is an internal Agent implementation surface rather than a general public API of the `prw-agent` library crate.

## Visibility decision

These modules become crate-visible only:

- `admission`;
- `responder`;
- `policy_response`.

The change is intentionally at the module boundary. Their internal item visibility can remain unchanged because an external crate cannot name a private module path.

## Security effect

This removes an accidental external path where another crate could create its own permissive `PolicyEvaluator` and invoke the Agent admission/response composition directly.

This is defense-in-depth, not authentication. Internal Agent code still has access and must eventually obey the authenticated-peer-before-policy ordering locked by Phases 037–038.

## Compatibility

No frame layout, command/status code, payload codec, capability mapping, tracker rule, or response semantics changes.

## Validation

Workspace compilation/tests ensure all current internal users remain reachable while the library no longer exports these modules publicly.
