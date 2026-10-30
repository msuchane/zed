use crate::tasks::workflows::{
    run_bundling::bundle_snap,
    runners::Arch,
    steps::{
        self, CommonPermissionSets, DEFAULT_REPOSITORY_OWNER_GUARD, FluentBuilder, IfNoFilesFound,
        named,
    },
};

use gh_workflow::*;
use indoc::indoc;

/// Generates the snap_build.yml workflow: a manual, snap-only build of both architectures that
/// uploads artifacts (the snaps plus Snap Store review evidence) and nothing else.
pub fn snap_build() -> Workflow {
    named::workflow()
        .with_minimal_permissions()
        .on(Event::default().workflow_dispatch(WorkflowDispatch::default()))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", "1"))
        .map(|mut workflow| {
            for arch in [Arch::X86_64, Arch::AARCH64] {
                let mut snap = bundle_snap(arch, &[]);
                snap.job = snap
                    .job
                    .cond(Expression::new(DEFAULT_REPOSITORY_OWNER_GUARD))
                    .add_step(lint_snap())
                    .add_step(collect_review_evidence())
                    .add_step(upload_review_evidence(arch));
                workflow = workflow.add_job(snap.name, snap.job);
            }
            workflow
        })
}

// `snapcraft lint` exits non-zero when it reports warnings, which are expected and are
// explained in classic-justification.md, so record the exit status instead of failing.
fn lint_snap() -> Step<Run> {
    named::bash(LINT_SNAP)
}

fn collect_review_evidence() -> Step<Run> {
    named::bash(COLLECT_REVIEW_EVIDENCE)
}

fn upload_review_evidence(arch: Arch) -> Step<Use> {
    steps::upload_artifact(format!("zed-snap-review-{arch}"), "snap-review/*")
        .if_no_files_found(IfNoFilesFound::Error)
        .into()
}

const LINT_SNAP: &str = indoc! {r#"
    mkdir -p snap-review
    status=0
    snapcraft lint zed_*_${SNAP_ARCH}.snap 2>&1 | tee "snap-review/snapcraft-lint-${SNAP_ARCH}.txt" || status=$?
    echo "snapcraft lint exit status: ${status}" | tee -a "snap-review/snapcraft-lint-${SNAP_ARCH}.txt"
"#};

const COLLECT_REVIEW_EVIDENCE: &str = indoc! {r#"
    cp crates/zed/resources/snap/classic-justification.md snap-review/classic-justification.md
    cp snap/snapcraft.yaml snap-review/snapcraft.yaml
    for name in code sublime-text intellij-idea-community pycharm-community nvim; do
        curl -fsS -H 'Snap-Device-Series: 16' \
            "https://api.snapcraft.io/v2/snaps/info/${name}?fields=confinement" |
            jq -r '"\(.name): \(.["channel-map"][0].confinement)"'
    done | tee snap-review/classic-precedent.txt
"#};
