#!/bin/bash
#
# Fluxx Agent — install entry point.
#
# This URL (https://get.fluxx.vexarr.com) is intentionally locked
# while Fluxx is in private beta. Public installs would otherwise
# pull binaries from a release pipeline that is not yet stable enough
# for self-service onboarding.
#
# If you're a beta participant: follow the install instructions in
# your invite email instead of running this script. They use a
# different, account-scoped install path.
#
# If you arrived here from a search result or a stale link: please
# come back when we open. We'll announce on https://fluxx.vexarr.com.
#
# Build: 2026-05-10  closed-beta-refusal

set -e

red()  { printf '\033[31m%s\033[0m\n' "$*" >&2; }
bold() { printf '\033[1m%s\033[0m\n' "$*" >&2; }
dim()  { printf '\033[2m%s\033[0m\n' "$*" >&2; }

bold "Fluxx is in private beta — public installs are closed."
echo  >&2
dim   "  • Beta participant?"
echo  "    Follow the install instructions in your invite email." >&2
echo  "    They use an account-scoped path that does not run this script." >&2
echo  >&2
dim   "  • Not a participant yet?"
echo  "    We'll announce general availability on https://fluxx.vexarr.com." >&2
echo  "    For early access requests: support@fluxx.vexarr.com" >&2
echo  >&2
red   "Refusing to install. No changes were made to this system."
exit 1
